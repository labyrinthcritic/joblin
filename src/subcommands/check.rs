use colored::Colorize;

use crate::job::Jobs;

pub fn check(jobs: &Jobs) {
    let job_count = jobs.jobs.len();

    println!(
        "{} {} found.",
        job_count,
        if job_count == 1 {
            "job was"
        } else {
            "jobs were"
        }
    );

    for job in jobs.jobs.iter() {
        println!("\t{}", job.name.bright_green());
    }
}
