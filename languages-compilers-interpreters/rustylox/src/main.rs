#[macro_use]
extern crate enum_display_derive;

use ast::Expr;
use clap::{App, Arg, SubCommand};
use error::{error, ErrorKind};
use evaluator::evaluate;
use parser::Parser;
use scanner::{default_reserved, Scanner};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use token::{Token, TokenType};

mod ast;
mod error;
mod evaluator;
mod parser;
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
            for token in &tokens {
                print!(" ({}) ", token);
            }
            print!("]\n");
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    println!("{}", ast);
                    println!("result: {:?}", evaluate(ast));
                }

                Err(_) => println!("{}", "invalid expression"),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn run_ast() {
    // -123 * 45.67
    let expr = Expr::Binary(
        Box::new(Expr::Unary(
            Token::new(TokenType::Minus, String::from("-"), 0, 0, None),
            Box::new(Expr::NumberLiteral(123 as f64)),
        )),
        Token::new(TokenType::Star, String::from("*"), 0, 0, None),
        Box::new(Expr::Grouping(Box::new(Expr::NumberLiteral(45.67)))),
    );
    println!("{}", expr);
    println!("result: {:?}", evaluate(expr));
}

fn main() {
    let m = App::new("rustylox")
        .version("0.0.1")
        .about("Rust interpreter for the Lox language")
        .arg(Arg::with_name("input").index(1))
        .subcommand(SubCommand::with_name("ast"))
        .get_matches();

    if let Some(_) = m.subcommand_matches("ast") {
        run_ast();
    } else if m.is_present("input") {
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
