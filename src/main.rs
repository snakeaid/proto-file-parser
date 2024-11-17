use proto_file_parser::Proto;
use std::path::PathBuf;
use std::env;

const HELP: &str = r#"
Usage: proto-file-parser <COMMAND> [OPTIONS]

Commands:
  parse <FILE>              Parse a protobuf file and output JSON
    Options:
      -o, --output <FILE>   Output file (optional, defaults to stdout)
      -p, --pretty         Pretty print the JSON output

  help                     Show grammar guide and usage information
  credits                  Show project credits and information

Examples:
  Parse a protobuf file:
    proto-file-parser parse input.proto

  Parse and save as pretty-printed JSON:
    proto-file-parser parse input.proto -p -o output.json

  Show credits:
    proto-file-parser credits

  Show help:
    proto-file-parser help
"#;

const CREDITS: &str = r#"
Proto File Parser v0.1.0

Author:
  Illia Antypenko <i.antypenko@ukma.edu.ua>

Repository:
  https://github.com/snakeaid/proto-file-parser

Built with:
  - Rust Programming Language
  - pest (Parser Generator)
  - serde (Serialization)

License: MIT
"#;

#[derive(Debug)]
enum Command {
    Parse {
        file: PathBuf,
        output: Option<PathBuf>,
        pretty: bool,
    },
    Help,
    Credits,
}

fn parse_args() -> Result<Command, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No command provided. Use 'help' for usage information.".to_string());
    }

    match args[0].as_str() {
        "help" => Ok(Command::Help),
        "credits" => Ok(Command::Credits),
        "parse" => {
            if args.len() < 2 {
                return Err("No input file provided for parse command.".to_string());
            }
            let mut output = None;
            let mut pretty = false;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "-o" | "--output" => {
                        if i + 1 >= args.len() {
                            return Err("No output file provided after -o/--output".to_string());
                        }
                        output = Some(PathBuf::from(&args[i + 1]));
                        i += 2;
                    }
                    "-p" | "--pretty" => {
                        pretty = true;
                        i += 1;
                    }
                    _ => {
                        return Err(format!("Unknown option: {}", args[i]));
                    }
                }
            }
            Ok(Command::Parse {
                file: PathBuf::from(&args[1]),
                output,
                pretty,
            })
        }
        cmd => Err(format!("Unknown command: {}. Use 'help' for usage information.", cmd)),
    }
}

fn show_help() {
    println!("{}", HELP);
}

fn show_credits() {
    println!("{}", CREDITS);
}

fn main() {
    let command = match parse_args() {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match command {
        Command::Parse { file, output, pretty } => {
            match Proto::parse_file(file.to_str().unwrap()) {
                Ok(json) => {
                    let result = if pretty {
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

        Command::Help => {
            show_help();
        }

        Command::Credits => {
            show_credits();
        }
    }
}