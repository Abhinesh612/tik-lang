mod error;
mod token_type;
mod token;
mod scanner;
mod expr;
mod astprinter;
mod parser;

use error::*;
use scanner::*;
use expr::*;
use astprinter::*;

use std::io::{self, stdout, Write, BufRead};
use std::fs;
use std::env::args;

use crate::token::{Object, Token};
use crate::token_type::TokenType;


fn main() {

    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token {
                ttype: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        })),
        operator: Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(45.6)),
            }))
        }))
    });

    let printer = AstPrinter {};
    println!("{}", printer.print(&expression).unwrap());

    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);

    if args.len() > 2 {
        println!("Usage: tik [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("run_file failed");
    } else {
        run_prompt();
    }
}


fn run_file(path: &String) -> io::Result<()>{
    let buffer = fs::read_to_string(path)?;

    match run(buffer) {
        Ok(_) => {},
        Err(_) => {
            std::process::exit(65);
        }
    }
    
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();

    print!("> ");
    let _ = stdout().flush();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {},
                Err(_) => {
                },
            }
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), TikError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("Token: {:?}", token);
    }

    Ok(())
}

