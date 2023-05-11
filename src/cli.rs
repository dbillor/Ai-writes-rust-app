use crate::log_parser::{LogEntry, parse_log};
use crate::logger::Logger;

pub fn run() {
    // TODO: Implement command-line interface logic
    let log_file = "path/to/log/file";
    let input_log = "sample log content";
    let log_entries = parse_log(input_log);
    let logger = Logger::new(log_file);
    logger.write_log(&log_entries);
}