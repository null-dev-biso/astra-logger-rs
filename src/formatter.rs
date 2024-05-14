use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub enum LogFormatter {
    Info,
    Warning,
    Error,
    Trace,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub level: LogFormatter,
    pub message: String,
}

pub struct Logs {
    pub total_messages: usize,
    pub info_messages: Vec<String>,
    pub warning_messages: Vec<String>,
    pub error_messages: Vec<String>,
    pub trace_messages: Vec<String>,
}

impl Logs {
    pub fn new() -> Logs {
        Logs {
            total_messages: 0,
            info_messages: Vec::new(),
            warning_messages: Vec::new(),
            error_messages: Vec::new(),
            trace_messages: Vec::new(),
        }
    }

    pub fn analyze_log_line(&mut self, line: &str) {
        self.total_messages += 1;

        let info_regex = Regex::new(r"(?i)info").unwrap();
        let warning_regex = Regex::new(r"(?i)warning").unwrap();
        let error_regex = Regex::new(r"(?i)error").unwrap();

        if info_regex.is_match(line) {
            self.info_messages.push(line.to_string());
        } else if warning_regex.is_match(line) {
            self.warning_messages.push(line.to_string());
        } else if error_regex.is_match(line) {
            self.error_messages.push(line.to_string());
        } else {
            self.trace_messages.push(line.to_string());
        }
    }

    pub fn format_to_json(&self, output_path: &str) -> std::io::Result<()> {
        let mut entries: Vec<LogEntry> = Vec::new();

        for message in &self.info_messages {
            entries.push(LogEntry {
                level: LogFormatter::Info,
                message: message.clone(),
            });
        }

        for message in &self.warning_messages {
            entries.push(LogEntry {
                level: LogFormatter::Warning,
                message: message.clone(),
            });
        }

        for message in &self.error_messages {
            entries.push(LogEntry {
                level: LogFormatter::Error,
                message: message.clone(),
            });
        }

        for message in &self.trace_messages {
            entries.push(LogEntry {
                level: LogFormatter::Trace,
                message: message.clone(),
            });
        }

        let json = serde_json::to_string_pretty(&entries)?;

        let mut file = File::create(output_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}
