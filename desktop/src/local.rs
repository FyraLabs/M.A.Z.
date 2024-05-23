use std::{env::VarError, path::PathBuf};

lazy_static::lazy_static! {
    static ref HOMEDIR: PathBuf = entail_dir(homedir::get_my_home().unwrap().unwrap());
    static ref DATADIR: PathBuf = entail_dir(determine_datadir());
}

fn entail_dir(p: PathBuf) -> PathBuf {
    if !p.exists() {
        panic!(
            "Tried to find datadir: {}: no such file or directory",
            p.display()
        );
    }
    if !p.is_dir() {
        panic!("Tried to find datadir: {}: is not a directory", p.display());
    }
    p.canonicalize().expect("Cannot canonicalize datadir")
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
