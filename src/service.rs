use std::process::{Command, Stdio};
use std::path::PathBuf;

use crate::{
    proc_info::{self, is_running, kill},
    rutas::executor_path,
};

const SERVICE_NAME: &str = "ptech_dns_execu";

pub async fn start() -> anyhow::Result<()> {
    if is_running(SERVICE_NAME) {
        kill(SERVICE_NAME);
    }

    let bin = executor_path();
    let bin_folder: PathBuf = bin.parent().unwrap().into();

    // Ejecutar el proceso con salida en la consola
    Command::new(&bin)
        .current_dir(&bin_folder)
        .stdin(Stdio::null())        // no necesita input
        .stdout(Stdio::inherit())    // imprime EN TU CONSOLA
        .stderr(Stdio::inherit())    // imprime errores EN TU CONSOLA
        .spawn()?;                   // no bloquea

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
