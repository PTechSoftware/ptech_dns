// src/paths.rs
use std::{fs, path::PathBuf};

pub fn config_dir() -> PathBuf {
    let cfg = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("domainhdlr");

    if cfg.exists() ==false {
        _ =fs::create_dir(&cfg);
    }
    cfg
}

pub fn config_file() -> PathBuf {
    config_dir().join("domainhdlr.json")
}

pub fn log_file_error() -> PathBuf {
    config_dir().join("log_error.txt")
}

pub fn log_file() -> PathBuf {
    config_dir().join("log.txt")
}
pub fn bin_dir() -> PathBuf {
    let bin =    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local/bin");
    if bin.exists() ==false {
        _ =fs::create_dir(&bin);
    }
    bin
}

pub fn bin_path() -> PathBuf {
    bin_dir().join("domainhdlr")
}

pub fn systemd_user_dir() -> PathBuf {
    PathBuf::from("/etc/systemd/system")
}

pub fn service_path() -> PathBuf {
    systemd_user_dir().join("domainhdlr.service")
}

pub fn executor_path() -> PathBuf {
    bin_dir().join("executor")
}