use super::log_job::schedule_log_cleanup;

pub fn initialize_job() {
    log::info!("Initializing jobs...");

    // Schedule log cleanup job
    schedule_log_cleanup();

    log::info!("Jobs initialized");
}
