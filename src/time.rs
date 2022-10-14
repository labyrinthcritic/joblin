use chrono::{Datelike, Timelike};

use crate::job::Time;

// determine if the current time fits a job's time requirement
pub fn check_time(job_time: &Option<Time>) -> bool {
    if let Some(job_time) = job_time {
        let date_time = chrono::offset::Local::now();

        if let Some(month) = &job_time.month {
            if !matches_any(&date_time.month(), month) {
                return false;
            }
        }

        if let Some(day) = &job_time.day {
            if !matches_any(&date_time.day(), day) {
                return false;
            }
        }

        if let Some(weekday) = &job_time.weekday {
            if !matches_any(&date_time.weekday().num_days_from_monday(), weekday) {
                return false;
            }
        }

        if let Some(hour) = &job_time.hour {
            if !matches_any(&date_time.hour(), hour) {
                return false;
            }
        }

        if let Some(minute) = &job_time.minute {
            if !matches_any(&date_time.minute(), minute) {
                return false;
            }
        }
    }

    true
}

fn matches_any<T: PartialEq>(item: &T, list: &[T]) -> bool {
    list.iter().any(|elem| elem == item)
}
