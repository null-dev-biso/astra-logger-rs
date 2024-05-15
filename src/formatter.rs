use chrono::{DateTime, Local};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
    pub date: String, // Изменили тип на String
    pub file_path: PathBuf,
}

impl LogEntry {
    pub fn new(
        level: LogFormatter,
        message: String,
        date: DateTime<Local>,
        file_path: PathBuf,
    ) -> Self {
        LogEntry {
            level,
            message,
            date: date.to_rfc3339(),
            file_path,
        }
    }
}

pub struct Logs {
    pub total_messages: usize,
    pub entries: Vec<LogEntry>,
}

impl Logs {
    pub fn new() -> Logs {
        Logs {
            total_messages: 0,
            entries: Vec::new(),
        }
    }

    pub async fn analyze_log_line(&mut self, line: &str, file_path: PathBuf) {
        self.total_messages += 1;

        let info_regex = Regex::new(r"(?i)info").unwrap();
        let warning_regex = Regex::new(r"(?i)warning").unwrap();
        let error_regex = Regex::new(r"(?i)error").unwrap();

        let date = Local::now();

        if info_regex.is_match(line) {
            self.entries.push(LogEntry::new(
                LogFormatter::Info,
                line.to_string(),
                date,
                file_path,
            ));
        } else if warning_regex.is_match(line) {
            self.entries.push(LogEntry::new(
                LogFormatter::Warning,
                line.to_string(),
                date,
                file_path,
            ));
        } else if error_regex.is_match(line) {
            self.entries.push(LogEntry::new(
                LogFormatter::Error,
                line.to_string(),
                date,
                file_path,
            ));
        } else {
            self.entries.push(LogEntry::new(
                LogFormatter::Trace,
                line.to_string(),
                date,
                file_path,
            ));
        }
    }

    pub async fn format_to_json(&self, output_path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)?;

        let mut file = File::create(output_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}
