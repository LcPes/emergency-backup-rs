use log::info;
use sysinfo::System;

pub struct CpuTracker {}

impl CpuTracker {
    pub fn new() -> Self {
        CpuTracker {}
    }

    pub fn start_cpu_tracker(&self) {
        let _ = std::thread::spawn(|| {
            let mut sys = System::new_all();
            let interval = std::time::Duration::from_secs(300);

            std::thread::sleep(std::time::Duration::from_secs(5));

            loop {
                let this_pid = sysinfo::get_current_pid().unwrap();
                sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[this_pid]), true);
                let this_process = sys.process(this_pid).unwrap();
                let cpu_usage = this_process.cpu_usage();

                let message = format!("(PID: {}) CPU usage: ({:.3})%", this_pid, cpu_usage);

                info!("{}", message);

                std::thread::sleep(interval);
            }
        });
    }
}
