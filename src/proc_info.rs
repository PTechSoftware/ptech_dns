use sysinfo::System;

pub fn kill(process_name: &str) {
    let mut s = System::new();
    s.refresh_processes(sysinfo::ProcessesToUpdate::All, false);

    for (pid, proc) in s.processes() {
        if proc.name() == process_name {
            let status = proc.kill_and_wait();
            if let Ok(k) = status {
                if let Some(stat) = k {
                    println!(
                        "Process id:{}, action kill ended with status : {}",
                        pid.as_u32(),
                        stat
                    )
                }
            }
        }
    }
}

pub fn is_running(process_name: &str) -> bool {
    let mut s = System::new();
    s.refresh_processes(sysinfo::ProcessesToUpdate::All, false);

    for (_, proc) in s.processes() {
        if proc.name() == process_name {
            return true;
        }
    }
    false
}

pub fn proc_information(process_name: &str) {
    let mut s = System::new();
    s.refresh_processes(sysinfo::ProcessesToUpdate::All, false);

    for (pid, proc) in s.processes() {
        if proc.name() == process_name {
            println!(r#"
            ------------------------------------------------------
            PID: {}
            CPU USAGE : {}
            MEMORY: {}
            RUN TIME : {}
            ------------------------------------------------------
            "#, 
                proc.accumulated_cpu_time(),
                proc.cpu_usage(),
                proc.memory(),
                proc.run_time())
        }
    }
}
