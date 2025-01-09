use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};

pub fn init_logger() {
    // Initialize the logger to write all logs to a single file
    Logger::try_with_env_or_str("info") // Use "info" level or take from environment variable
        .expect("Failed to initialize logger with log specification")
        .log_to_file(
            FileSpec::default()
                .directory("logs") // Logs will be saved in the "logs" directory
                .basename("application") // Log file name will be "logs/application.log"
                .suppress_timestamp(), // No timestamp in file names
        )
        .duplicate_to_stdout(Duplicate::Info) // Print Info and above logs to the terminal
        .write_mode(WriteMode::BufferAndFlush) // Buffer logs for better performance
        .format(|writer, now, record| {
            // Define custom log format
            write!(
                writer,
                "{} [{}] - {}",
                now.now().format("%Y-%m-%d %H:%M:%S"), // Timestamp
                record.level(),                        // Log level
                record.args()                          // Log message
            )
        })
        .append() // Append to the log file
        .start() // Start the logger
        .expect("Failed to start the logger"); // If failed then panic with this message
}
