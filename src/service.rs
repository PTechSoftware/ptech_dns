

use std::process::{Command};

use crate::{proc_info::{self, is_running, kill}, rutas::executor_path};

const SERVICE_NAME :&'static str = "ptech_dns_executor";

pub async fn start() -> anyhow::Result<()>{

    if is_running(SERVICE_NAME) {
        kill(SERVICE_NAME);
    }
    let bin = executor_path();
    let bin_f  = bin.clone();
    let bin_folder = bin_f.parent().unwrap();
    _ = Command::new(bin)
        .current_dir(bin_folder)
        .output()?;

    Ok(())
}



pub async fn stop() -> anyhow::Result<()>{
    if is_running(SERVICE_NAME) {
        kill(SERVICE_NAME);
    }
    Ok(())
}



pub async fn status() -> anyhow::Result<()>{
    proc_info::proc_information(SERVICE_NAME);
    Ok(())
}

pub async fn restart() -> anyhow::Result<()>{
    while is_running(SERVICE_NAME) {

        kill(SERVICE_NAME);
    }
    let bin = executor_path();
    let bin_f  = bin.clone();
    let bin_folder = bin_f.parent().unwrap();
    _ = Command::new(bin)
        .current_dir(bin_folder)
        .output()?;

    Ok(())
}