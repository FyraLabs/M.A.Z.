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

#[derive(Debug)]
pub enum Event {
    DecryptFailure(maz_auth::AuthErr),
    Init,
    IoFailure(std::io::Error),
}

pub fn read_password(name: &str) -> Option<String> {
    let entry = keyring::Entry::new(crate::APPID, name).ok()?;
    entry.get_password().ok()
}

pub fn read_offline_default_locker(password: &str) -> Result<maz_auth::Locker, Event> {
    let path = DATADIR.join("default.mazl");
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
