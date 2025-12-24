use std::env;
use std::path::PathBuf;
use std::process;

mod markdown;
mod site;
mod template;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match parse_args(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Try '{} --help' for more information.", NAME);
            process::exit(1);
        }
    };

    match config {
        Action::Help => print_help(),
        Action::Version => println!("{} {}", NAME, VERSION),
        Action::Build { input, output } => {
            println!("Building site...");
            if let Err(e) = site::build(&input, &output) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            println!("Done.");
        }
    }
}

enum Action {
    Help,
    Version,
    Build { input: PathBuf, output: PathBuf },
}

fn parse_args(args: &[String]) -> Result<Action, String> {
    let mut input = PathBuf::from("content");
    let mut output = PathBuf::from("dist");
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => return Ok(Action::Help),
            "-V" | "--version" => return Ok(Action::Version),
            "-i" | "--input" => {
                i += 1;
                if i >= args.len() {
                    return Err("--input requires a path".to_string());
                }
                input = PathBuf::from(&args[i]);
            }
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    return Err("--output requires a path".to_string());
                }
                output = PathBuf::from(&args[i]);
            }
            arg if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg));
            }
            path => {
                input = PathBuf::from(path);
            }
        }
        i += 1;
    }

    Ok(Action::Build { input, output })
}

fn print_help() {
    println!(
        "{} {} - Minimal static site generator

USAGE:
    {} [OPTIONS] [INPUT]

ARGS:
    [INPUT]    Input directory (default: content)

OPTIONS:
    -i, --input <DIR>     Input directory containing .md files
    -o, --output <DIR>    Output directory for .html files (default: dist)
    -h, --help            Print help information
    -V, --version         Print version information

EXAMPLES:
    {}                    Build content/ -> dist/
    {} ./docs             Build docs/ -> dist/
    {} -i src -o public   Build src/ -> public/",
        NAME, VERSION, NAME, NAME, NAME, NAME
    );
}
