use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

const JOBS_DIR: &str = ".config/jot/jobs";

#[derive(Debug)]
pub struct JobConfiguration {
    pub name: String,
    pub job_file: JobFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobFile {
    pub job: Job,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub time: Option<Time>,
    pub commands: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Time {
    pub minute: Option<Vec<u32>>,
    pub hour: Option<Vec<u32>>,
    pub day: Option<Vec<u32>>,
    pub month: Option<Vec<u32>>,
    pub weekday: Option<Vec<u32>>,
}

#[derive(Debug)]
pub struct Jobs {
    pub jobs: Vec<JobConfiguration>,
}

impl Jobs {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    /// Construct a new `Jobs` by reading a directory of job files.
    pub fn read(directory: Option<PathBuf>) -> Self {
        if let Some(home_dir) = dirs::home_dir() {
            let jobs_dir = directory
                .unwrap_or_else(|| [home_dir, JOBS_DIR.into()].iter().collect::<PathBuf>());

            if let Ok(read_dir) = std::fs::read_dir(jobs_dir) {
                let mut jobs = Vec::new();

                for item in read_dir.into_iter().flatten() {
                    if let Ok(metadata) = item.metadata() {
                        if metadata.is_file() {
                            if let Ok(file) = File::open(item.path()) {
                                let job_file = serde_yaml::from_reader::<_, JobFile>(file);
                                let name = item.file_name().to_string_lossy().to_string();

                                if let Ok(job_file) = job_file {
                                    jobs.push(JobConfiguration { name, job_file });
                                }
                            }
                        }
                    }
                }

                Self { jobs }
            } else {
                Self::new()
            }
        } else {
            Self::new()
        }
    }
}
