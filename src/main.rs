use clap::{Parser, Subcommand};
use proto_file_parser::Proto;
use std::path::PathBuf;

const CREDITS: &str = r#"
Proto File Parser v0.1.0
Created by: Illia Antypenko
License: MIT
Repository: https://github.com/snakeaid/proto-file-parser

This tool is built with:
- Rust Programming Language
- pest (Parser Generator)
- clap (CLI Framework)
- serde (Serialization)
"#;

#[derive(Parser)]
#[command(
    author = "Illia Antypenko i.antypenko@ukma.edu.ua",
    version,
    about = "A Proto File parser that converts .proto files to JSON format",
    long_about = None,
    after_help = "For more information, visit: https://github.com/snakeaid/proto-file-parser"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a protobuf file and output JSON
    Parse {
        /// Input protobuf file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output file (optional, defaults to stdout)
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Pretty print the JSON output
        #[arg(short, long)]
        pretty: bool,
    },

    /// Display version and credits information
    Credits,

    /// Validate a protobuf file without generating JSON
    Validate {
        /// Input protobuf file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file, output, pretty } => {
            match Proto::parse_file(file.to_str().unwrap()) {
                Ok(json) => {
                    let result = if *pretty {
                        match serde_json::from_str::<serde_json::Value>(&json) {
                            Ok(value) => serde_json::to_string_pretty(&value).unwrap(),
                            Err(_) => json,
                        }
                    } else {
                        json
                    };

                    match output {
                        Some(path) => {
                            if let Err(e) = std::fs::write(path, result) {
                                eprintln!("Error writing to file: {}", e);
                                std::process::exit(1);
                            }
                        }
                        None => println!("{}", result),
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing file: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Credits => {
            println!("{}", CREDITS);
        }

        Commands::Validate { file } => {
            match Proto::parse_file(file.to_str().unwrap()) {
                Ok(_) => {
                    println!("✓ File is valid");
                }
                Err(e) => {
                    eprintln!("✗ Validation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}