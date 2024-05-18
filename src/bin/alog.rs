use astra_logger_rs::formatter::Logs;
use astra_logger_rs::scanner::LogStats;
use astra_logger_rs::vizualizer::run_app;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Путь к файлу или директории с логами
    #[arg(short, long)]
    paths: Vec<PathBuf>,
    /// Регулярное выражение для фильтрации строк логов
    #[arg(short = 'l', long, default_value = "")]
    pattern: String,

    /// Сохранение файла для удобного формата логов в json
    #[arg(short = 'j', long)]
    output_json: Option<String>,
    /// Запуск TUI
    #[arg(short = 't', long)]
    tui: bool,
}

#[tokio::main]

async fn main() {
    let args = Args::parse();

    let mut log_stats = LogStats::new();
    let mut formatter = Logs::new();

    if args.paths.is_empty() {
        eprintln!("No paths provided");
        return;
    }

    for path in &args.paths {
        if path.is_file() {
            analyze_file(path, &mut log_stats, &mut formatter, &args.pattern).await;
        } else if path.is_dir() {
            analyze_directory(path, &mut log_stats, &mut formatter, &args.pattern).await;
        } else {
            eprintln!("Invalid path: {}", path.display());
            return;
        }
    }

    if args.tui {
        if let Err(err) = run_app(formatter, log_stats) {
            eprintln!("Error running TUI: {}", err);
        }
    } else {
        log_stats.print_stats();

        if let Some(output_path) = args.output_json {
            if let Err(err) = formatter.format_to_json(&output_path).await {
                eprintln!("Error formatting log stats to JSON: {}", err);
            } else {
                println!("Log stats formatted to JSON and saved to {}", output_path);
            }
        }
    }
}

async fn analyze_file(
    path: &PathBuf,
    log_stats: &mut LogStats,
    formatter: &mut Logs,
    pattern: &str,
) {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if pattern.is_empty() || line.contains(pattern) {
                    formatter.analyze_log_line(&line, path.clone()).await;
                    log_stats.analyze_log_line(&line).await;
                }
            }
        }
    } else {
        eprintln!("Failed to open log file: {}", path.display());
    }
}

async fn analyze_directory(
    path: &PathBuf,
    log_stats: &mut LogStats,
    formatter: &mut Logs,
    pattern: &str,
) {
    for entry in path.read_dir().expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            analyze_file(&path, log_stats, formatter, pattern).await;
        }
    }
}
