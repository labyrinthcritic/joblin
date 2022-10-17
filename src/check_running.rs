use colored::Colorize;
use sysinfo::{PidExt, ProcessExt, SystemExt};

pub fn check_running() {
    let mut system = sysinfo::System::new_all();

    system.refresh_all();

    let this_pid = std::process::id();

    for (pid, process) in system.processes().iter() {
        if let Some(process_name) = process.exe().file_name().map(|name| name.to_string_lossy()) {
            if process_name == "joblin" && pid.as_u32() != this_pid {
                eprintln!(
                    "{} joblin is already running.",
                    "error:".color(colored::Color::BrightRed)
                );

                std::process::exit(1);
            }
        }
    }
}
