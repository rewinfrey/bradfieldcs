#[macro_use]
extern crate enum_display_derive;

use ast::{Expr, Stmt};
use clap::{App, Arg, SubCommand};
use environment::Environment;
use error::{error, ErrorKind};
use interpreter::Interpreter;
use parser::Parser;
use scanner::{default_reserved, Scanner};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use token::{Token, TokenType};
use value::Value;

mod ast;
mod environment;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod value;

fn run_file(path: &Path) -> Result<(), ErrorKind> {
    let env = Environment::<Value>::new();
    let mut interpreter = Interpreter::new(env);
    match fs::read_to_string(&path) {
        Ok(source) => {
            run(source, &mut interpreter);
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
    let env = Environment::<Value>::new();
    let mut interpreter = &mut Interpreter::new(env);

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
                interpreter = run(line.to_string(), interpreter);
            }
            Err(e) => {
                println!("{} error", e);
            }
        }
    }
}

fn run(source: String, interpreter: &mut Interpreter<Value>) -> &mut Interpreter<Value> {
    let mut scanner = Scanner::new(default_reserved(), source.as_str());
    match scanner.scan_tokens() {
        Ok(tokens) => {
            print!("Tokens:\n[");
            for token in &tokens {
                print!(" ({}) ", token);
            }
            print!("]\n\n");

            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(stmts) => {
                    println!("Parsed:");
                    println!("{:?}\n", stmts);

                    println!("Result:");
                    match interpreter.evaluate(&stmts) {
                        Ok(value) => {
                            println!("{}", value);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Err(e) => eprintln!("{}", e),
    }
    interpreter
}

fn run_ast() {
    // -123 * 45.67
    let env = Environment::<Value>::new();
    let expr = Stmt::ExprStmt(Expr::Binary(
        Box::new(Expr::Unary(
            Token::new(TokenType::Minus, String::from("-"), 0, 0, None),
            Box::new(Expr::NumberLiteral(123 as f64)),
        )),
        Token::new(TokenType::Star, String::from("*"), 0, 0, None),
        Box::new(Expr::Grouping(Box::new(Expr::NumberLiteral(45.67)))),
    ));
    println!("{:?}", expr);
    let mut interpreter = Interpreter::new(env);
    println!("result: {:?}", interpreter.evaluate(&vec![expr]));
}

fn main() {
    let m = App::new("rustylox")
        .version("0.0.1")
        .about("Rust interpreter for the Lox language")
        .subcommand(SubCommand::with_name("ast"))
        .subcommand(SubCommand::with_name("repl"))
        .arg(Arg::with_name("input").index(1))
        .get_matches();

    if let Some(_) = m.subcommand_matches("ast") {
        return run_ast();
    }

    if let Some(_) = m.subcommand_matches("repl") {
        return run_repl();
    }

    if m.is_present("input") {
        if let Some(file_path) = m.value_of("input") {
            std::process::exit(match run_file(Path::new(file_path)) {
                Ok(_) => 0,
                Err(_) => 65,
            });
        }
    }

    run_repl();
}
