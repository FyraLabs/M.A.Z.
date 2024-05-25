use std::{env::VarError, path::PathBuf};

lazy_static::lazy_static! {
    static ref HOMEDIR: PathBuf = entail_dir(homedir::get_my_home().unwrap().unwrap());
    static ref DATADIR: PathBuf = entail_dir(determine_datadir().join(crate::APPID));
}

#[tracing::instrument]
fn entail_dir(p: PathBuf) -> PathBuf {
    if !p.exists() {
        std::fs::create_dir_all(&p).expect("Cannot create datadir");
    }
    if !p.is_dir() {
        panic!("Tried to find dir: {}: is not a directory", p.display());
    }
    p.canonicalize().expect("Cannot canonicalize directory")
}

fn determine_datadir() -> PathBuf {
    if cfg!(target_os = "macos") {
        HOMEDIR.join(PathBuf::from("Library/Application Support"))
    } else if cfg!(unix) {
        match std::env::var("XDG_DATA_HOME") {
            Ok(dir) => PathBuf::from(dir),
            Err(VarError::NotPresent) => HOMEDIR.join(PathBuf::from(".local/share/")),
            Err(VarError::NotUnicode(_)) => panic!("$XDG_DATA_HOME is not unicode"),
        }
    } else if cfg!(windows) {
        match std::env::var("LOCALAPPDATA") {
            Ok(dir) => PathBuf::from(dir),
            Err(VarError::NotPresent) => HOMEDIR.join(PathBuf::from("AppData/Local")),
            Err(VarError::NotUnicode(_)) => panic!("%LOCALAPPDATA% is not unicode"),
        }
    } else {
        #[cfg(not(any(unix, windows)))]
        compile_error!("This OS is not supported");
        unreachable!("This OS is not supported")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Event {
    #[error("Fail to decrypt M.A.Z. locker: {0}")]
    DecryptFailure(#[from] maz_auth::AuthErr),
    #[error("BUG: Initialization required. (You should not be able to see this message)")]
    Init,
    #[error("Fail to access file: {0}")]
    IoFailure(#[from] std::io::Error),
}

pub fn list_lockers() -> Result<Vec<String>, Event> {
    let file_to_lockername = |f: std::fs::DirEntry| {
        (f.file_name().to_string_lossy().strip_suffix(".mazl")).map(ToString::to_string)
    };
    let entries = std::fs::read_dir(&*DATADIR)?.filter_map(Result::ok);
    let files = entries.filter(|f| f.metadata().is_ok_and(|meta| meta.is_file()));
    let lockers = files.filter_map(file_to_lockername);
    Ok(lockers.collect())
}

pub fn read_password(name: &str) -> Option<String> {
    let entry = keyring::Entry::new(crate::APPID, name).ok()?;
    entry.get_password().ok()
}

pub fn read_offline_locker(name: &str, password: &str) -> Result<maz_auth::Locker, Event> {
    let path = DATADIR.join(format!("{name}.mazl"));
    let f = std::fs::File::open(&path);
    let Ok(mut f) = f else {
        let e = f.unwrap_err();
        if matches!(e.kind(), std::io::ErrorKind::NotFound) {
            return Err(Event::Init);
        }
        return Err(Event::IoFailure(e));
    };
    maz_auth::Locker::from_file(&mut f, password.as_bytes()).map_err(|e| match e {
        maz_auth::AuthErr::Io(e) => Event::IoFailure(e),
        e => Event::DecryptFailure(e),
    })
}
