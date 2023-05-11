use crate::log_parser::LogEntry;

pub struct Logger {
    pub log_file: String,
}

impl Logger {
    pub fn new(log_file: &str) -> Self {
        Logger {
            log_file: log_file.to_string(),
        }
    }

    pub fn write_log(&self, log_entries: &[LogEntry]) {
        // TODO: Implement writing logs to a specified location
    }
}