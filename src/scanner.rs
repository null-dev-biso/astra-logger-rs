use regex::Regex;

#[derive(Clone)]
pub struct LogStats {
    pub total_messages: usize,
    pub info_messages: usize,
    pub warning_messages: usize,
    pub error_messages: usize,
    pub trace_messages: usize,
}

impl LogStats {
    pub fn new() -> LogStats {
        LogStats {
            total_messages: 0,
            info_messages: 0,
            warning_messages: 0,
            error_messages: 0,
            trace_messages: 0,
        }
    }

    pub async fn analyze_log_line(&mut self, line: &str) {
        self.total_messages += 1;

        let info_regex = Regex::new(r"(?i)info").unwrap();
        let warning_regex = Regex::new(r"(?i)warning").unwrap();
        let error_regex = Regex::new(r"(?i)error").unwrap();
        let trace_regex = Regex::new(r"(?i)").unwrap();

        if info_regex.is_match(line) {
            self.info_messages += 1;
        } else if warning_regex.is_match(line) {
            self.warning_messages += 1;
        } else if error_regex.is_match(line) {
            self.error_messages += 1;
        } else if trace_regex.is_match(line) {
            self.trace_messages += 1;
        }
    }

    pub fn print_stats(&self) {
        println!("Total messages: {}", self.total_messages);
        println!("Info messages: {}", self.info_messages);
        println!("Warning messages: {}", self.warning_messages);
        println!("Error messages: {}", self.error_messages);
        println!("Trace messages: {}", self.trace_messages);
    }
}
