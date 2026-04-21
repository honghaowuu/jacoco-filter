use std::fs;
use std::io::{self, Write};

use clap::Parser;

#[derive(Parser)]
#[command(name = "jacoco-filter", about = "Filter and score JaCoCo XML coverage reports")]
struct Cli {
    /// JaCoCo XML input file
    input_file: String,

    /// Output JSON file (defaults to stdout)
    #[arg(long)]
    output: Option<String>,

    /// Minimum score threshold (methods below this are excluded)
    #[arg(long, default_value = "0.0")]
    min_score: f64,

    /// Pretty-print JSON output
    #[arg(long)]
    pretty: bool,

    /// Include a line-coverage summary alongside the filtered methods
    #[arg(long)]
    summary: bool,

    /// Limit output to the top-k highest-scoring methods (0 = no limit)
    #[arg(long, default_value = "5")]
    top_k: usize,
}

fn main() {
    let cli = Cli::parse();

    let xml = match fs::read_to_string(&cli.input_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", cli.input_file, e);
            std::process::exit(1);
        }
    };

    let json = if cli.summary {
        let mut report = match jacoco_filter::process_with_summary(&xml, cli.min_score) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error processing XML: {}", e);
                std::process::exit(1);
            }
        };
        if cli.top_k > 0 {
            report.methods.truncate(cli.top_k);
        }
        if cli.pretty {
            serde_json::to_string_pretty(&report)
        } else {
            serde_json::to_string(&report)
        }
    } else {
        let mut methods = match jacoco_filter::process(&xml, cli.min_score) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error processing XML: {}", e);
                std::process::exit(1);
            }
        };
        if cli.top_k > 0 {
            methods.truncate(cli.top_k);
        }
        if cli.pretty {
            serde_json::to_string_pretty(&methods)
        } else {
            serde_json::to_string(&methods)
        }
    };

    let json = match json {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error serializing JSON: {}", e);
            std::process::exit(1);
        }
    };

    match &cli.output {
        Some(path) => {
            if let Err(e) = fs::write(path, &json) {
                eprintln!("Error writing {}: {}", path, e);
                std::process::exit(1);
            }
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            if let Err(e) = writeln!(handle, "{}", json) {
                eprintln!("Error writing to stdout: {}", e);
                std::process::exit(1);
            }
        }
    }
}
