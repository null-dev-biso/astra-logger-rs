// test.rs
#[cfg(test)]
mod tests {

    use crate::formatter::{LogEntry, LogFormatter, Logs};
    use crate::scanner::LogStats;
    use crate::vizualizer::EllipticCurve;
    use chrono::Local;
    use std::path::PathBuf;

    // Unit Tests
    #[cfg(test)]
    mod unit {
        use super::*;

        #[test]
        fn test_log_entry_new() {
            let date = Local::now();
            let log_entry = LogEntry::new(
                LogFormatter::Info,
                String::from("Test message"),
                date,
                PathBuf::from("/test/path"),
            );

            assert_eq!(log_entry.message, "Test message");
            assert_eq!(log_entry.date, date.to_rfc3339());
            assert_eq!(log_entry.file_path, PathBuf::from("/test/path"));
        }

        #[test]
        fn test_logs_new() {
            let logs = Logs::new();
            assert_eq!(logs.total_messages, 0);
            assert!(logs.entries.is_empty());
        }

        #[test]
        fn test_log_stats_new() {
            let stats = LogStats::new();
            assert_eq!(stats.total_messages, 0);
            assert_eq!(stats.info_messages, 0);
            assert_eq!(stats.warning_messages, 0);
            assert_eq!(stats.error_messages, 0);
            assert_eq!(stats.trace_messages, 0);
        }

        #[test]
        fn test_elliptic_curve_calculate_points() {
            let curve = EllipticCurve::new(1.0, -1.0);
            let points = curve.calculate_points(-2.0, 2.0, 0.5);
            assert!(!points.is_empty());
        }
    }

    // System Tests
    #[cfg(test)]
    mod system {
        use super::*;

        #[tokio::test]
        async fn test_analyze_log_line() {
            let mut logs = Logs::new();
            let file_path = PathBuf::from("/test/path");

            logs.analyze_log_line("Info: This is an info message", file_path.clone())
                .await;
            assert_eq!(logs.total_messages, 1);

            logs.analyze_log_line("Warning: This is a warning message", file_path.clone())
                .await;
            assert_eq!(logs.total_messages, 2);

            logs.analyze_log_line("Error: This is an error message", file_path.clone())
                .await;
            assert_eq!(logs.total_messages, 3);

            logs.analyze_log_line("This is a trace message", file_path.clone())
                .await;
            assert_eq!(logs.total_messages, 4);
        }

        #[tokio::test]
        async fn test_format_to_json() {
            let logs = Logs::new();
            let output_path = "/tmp/test_output.json";
            let result = logs.format_to_json(output_path).await;
            assert!(result.is_ok());

            let file_content = std::fs::read_to_string(output_path).unwrap();
            assert!(file_content.contains("[]"));
        }

        #[tokio::test]
        async fn test_log_stats_analyze_log_line() {
            let mut stats = LogStats::new();

            stats
                .analyze_log_line("Info: This is an info message")
                .await;
            assert_eq!(stats.total_messages, 1);
            assert_eq!(stats.info_messages, 1);

            stats
                .analyze_log_line("Warning: This is a warning message")
                .await;
            assert_eq!(stats.total_messages, 2);
            assert_eq!(stats.warning_messages, 1);

            stats
                .analyze_log_line("Error: This is an error message")
                .await;
            assert_eq!(stats.total_messages, 3);
            assert_eq!(stats.error_messages, 1);

            stats.analyze_log_line("This is a trace message").await;
            assert_eq!(stats.total_messages, 4);
            assert_eq!(stats.trace_messages, 1);
        }
    }
}
