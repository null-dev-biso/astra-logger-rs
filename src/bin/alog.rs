use astra_logger_rs::analyser::SystemInfo;
use astra_logger_rs::scanner::LogStats;
use clap::Parser;
use std::io::BufRead;
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

    #[arg(short = 's', long)]
    system_info: bool,
}

fn main() {
    let args = Args::parse();

    if args.system_info {
        print_system_info();
        return;
    }

    let mut log_stats = LogStats::new();

    if args.paths.is_empty() {
        eprintln!("No paths provided");
        return;
    }

    for path in &args.paths {
        if path.is_file() {
            analyze_file(path, &mut log_stats, &args.pattern);
        } else if path.is_dir() {
            analyze_directory(path, &mut log_stats, &args.pattern);
        } else {
            eprintln!("Invalid path: {}", path.display());
            return;
        }
    }

    log_stats.print_stats();
}

fn analyze_file(path: &PathBuf, log_stats: &mut LogStats, pattern: &str) {
    if let Ok(file) = std::fs::File::open(path) {
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if pattern.is_empty() || line.contains(pattern) {
                    log_stats.analyze_log_line(&line);
                }
            }
        }
    } else {
        eprintln!("Failed to open log file: {}", path.display());
    }
}

fn analyze_directory(path: &PathBuf, log_stats: &mut LogStats, pattern: &str) {
    for entry in path.read_dir().expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            analyze_file(&path, log_stats, pattern);
        }
    }
}

fn print_system_info() {
    let system_info = SystemInfo::new();

    println!("Total Memory: {} bytes", system_info.get_total_memory());
    println!("Free Memory: {} bytes", system_info.get_free_memory());
    println!("CPU Load: {:.2}%", system_info.get_cpu_load());
}
