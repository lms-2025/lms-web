use chrono::{DateTime, Utc};
use cron::Schedule;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

pub fn schedule_log_cleanup() {
    let schedule: Schedule = match Schedule::from_str("0 0 * * * *") {
        Ok(schedule) => schedule,
        Err(_) => {
            log::error!("Invalid cron expression for log cleanup job");
            return;
        }
    };

    tokio::spawn(async move {
        loop {
            let now: DateTime<Utc> = chrono::Utc::now();
            let next_run: DateTime<Utc> =
                schedule.upcoming(chrono::Utc).next().unwrap_or_else(|| {
                    log::error!("Error in calculating next run for log cleanup job");
                    chrono::Utc::now() + chrono::Duration::seconds(3600) // default to 1 hour later
                });

            if now < next_run {
                let sleep_duration: Duration = (next_run - now).to_std().unwrap_or_else(|_| {
                    log::error!("Error in calculating sleep duration for log cleanup job");
                    Duration::from_secs(3600) // default to 1 hour
                });
                sleep(sleep_duration).await;
            }

            // let log_cleanup = shared_log_cleanup.clone();
            // tokio::spawn(async move {
            //     let mut cleanup = log_cleanup.lock().await;
            //     cleanup.cleanup_logs().await;
            // });
        }
    });
}
