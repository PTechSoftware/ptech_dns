use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::{
    proc_info::{self, is_running, kill},
    rutas::executor_path,
};

const SERVICE_NAME: &str = "executor";

pub async fn start() -> anyhow::Result<()> {
    if is_running(SERVICE_NAME) {
        kill(SERVICE_NAME);
    }

    let bin = executor_path();
    let bin_folder: PathBuf = bin.parent().unwrap().into();

    // Ejecutar el proceso con salida en la consola
    Command::new(&bin)
        .current_dir(&bin_folder)
        .stdin(Stdio::null()) // no necesita input
        .stdout(Stdio::inherit()) // imprime EN TU CONSOLA
        .stderr(Stdio::inherit()) // imprime errores EN TU CONSOLA
        .spawn()?; // no bloquea

    Ok(())
}

pub fn start_detached() -> anyhow::Result<()> {
    let bin = executor_path();
    let bin_folder = bin.parent().unwrap();

    unsafe {
        Command::new(&bin)
            .current_dir(&bin_folder)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .pre_exec(|| {
                // async-signal-safe ONLY
                // setsid() SI ES async-signal-safe
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            })
            .spawn()?;
    }

    Ok(())
}
pub async fn stop() -> anyhow::Result<()> {
    if is_running(SERVICE_NAME) {
        kill(SERVICE_NAME);
    }
    Ok(())
}

pub async fn status() -> anyhow::Result<()> {
    proc_info::proc_information(SERVICE_NAME);
    Ok(())
}

pub async fn restart() -> anyhow::Result<()> {
    while is_running(SERVICE_NAME) {
        kill(SERVICE_NAME);
    }

    let bin = executor_path();
    let bin_folder: PathBuf = bin.parent().unwrap().into();

    Command::new(&bin)
        .current_dir(&bin_folder)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    Ok(())
}
