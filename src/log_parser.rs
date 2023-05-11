pub struct LogEntry {
    pub timestamp: String,
    pub user: String,
    pub action: String,
}

pub fn parse_log(log: &str) -> Vec<LogEntry> {
    let mut entries: Vec<LogEntry> = Vec::new();
    // TODO: Implement log parsing logic
    entries
}