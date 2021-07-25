#[macro_use]
extern crate enum_display_derive;

use clap::{App, Arg};
use error::{error, ErrorKind};
use scanner::{default_reserved, Scanner};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

mod error;
mod scanner;
mod token;

fn run_file(path: &Path) -> Result<(), ErrorKind> {
    match fs::read_to_string(&path) {
        Ok(source) => {
            run(source);
            Ok(())
        }
        Err(_) => {
            error(
                0,
                0,
                0,
                "error reading file".to_string(),
                ErrorKind::ReadFileError,
            );
            Err(ErrorKind::ReadFileError)
        }
    }
}

fn run_repl() {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(bytes) => {
                // Quit the repl if the user inputs an empty line or inputs 0 bytes as EOF (via ctrl-d).
                if line == "\n" || bytes == 0 {
                    break;
                }
                run(line.to_string());
            }
            Err(e) => {
                println!("{} error", e);
            }
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(default_reserved(), source.as_str());
    match scanner.scan_tokens() {
        Ok(tokens) => {
            print!("[");
            for token in tokens {
                print!(" ({}) ", token);
            }
            print!("]\n");
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn main() {
    let m = App::new("rustylox")
        .version("0.0.1")
        .about("Rust interpreter for the Lox language")
        .arg(Arg::with_name("input").index(1))
        .get_matches();

    if m.is_present("input") {
        if let Some(file_path) = m.value_of("input") {
            std::process::exit(match run_file(Path::new(file_path)) {
                Ok(_) => 0,
                Err(_) => 65,
            });
        }
    } else {
        run_repl();
    }
}
