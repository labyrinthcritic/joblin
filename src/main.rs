mod args;
mod check_running;
mod job;
mod subcommands;
mod time;

use chrono::Timelike;
use clap::Parser;

use args::{Args, Subcommand};
use job::{JobFile, Jobs};

fn main() {
    let args = Args::parse();

    let jobs = Jobs::read(args.jobs_dir.clone());

    check_running::check_running();

    if let Some(subcommand) = args.subcommand {
        match subcommand {
            Subcommand::Check => subcommands::check::check(&jobs),
        }
    } else {
        jobs_loop(&jobs);
    }
}

// main loop for checking and executing jobs each minute
fn jobs_loop(jobs: &Jobs) {
    loop {
        // wait until the next minute change

        let second = chrono::offset::Local::now().time().second();
        std::thread::sleep(std::time::Duration::from_secs(60 - second as u64));

        check_and_execute(jobs);
    }
}

fn check_and_execute(jobs: &Jobs) {
    for job_config in jobs.jobs.iter() {
        if time::check_time(&job_config.job_file.job.time) {
            execute_job(&job_config.job_file);
        }
    }
}

// unconditionally executes commands in the job file
fn execute_job(job_file: &JobFile) {
    for command in job_file.job.commands.iter() {
        // ignore if the command fails
        // `execute::shell` will run a full shell command from a string
        let _ = execute::shell(command).output();
    }
}
