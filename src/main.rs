mod error;
mod token_type;
mod token;
mod scanner;
mod expr;
mod astprinter;
mod parser;
mod interpreter;

use error::*;
use scanner::*;
use expr::*;
use astprinter::*;
use parser::*;
use token::*;
use interpreter::*;

use std::io::{self, stdout, Write, BufRead};
use std::fs;
use std::env::args;



fn main() {
    let args: Vec<String> = args().collect();
    let  mut tik = Tik::new();

    println!("args: {:?}", args);

    if args.len() > 2 {
        println!("Usage: tik [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        tik.run_file(&args[1]).expect("run_file failed");
    } else {
        tik.run_prompt();
    }
}


struct Tik {
    interpreter: Interpreter,
}

impl Tik {
    pub fn new() -> Tik {
        Tik { interpreter: Interpreter {} }
    }

    pub fn run_file(&self, path: &String) -> io::Result<()>{
        let buffer = fs::read_to_string(path)?;

        match self.run(buffer) {
            Ok(_) => {},
            Err(_) => {
                std::process::exit(65);
            }
        }

        Ok(())
    }

    pub fn run_prompt(&self) {
        let stdin = io::stdin();

        print!("> ");
        let _ = stdout().flush();

        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                match self.run(line) {
                    Ok(_) => {},
                    Err(_) => {
                    },
                }
            } else {
                break;
            }
        }
    }

    pub fn run(&self, source: String) -> Result<(), TikError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens.to_vec());

        match parser.parse() {
            Ok(expr) => {
                self.interpreter.interpret(&expr);
                let printer = AstPrinter {};
                let p = printer.print(&expr)?;
                println!("{}", p);
            },
            Err(_) => { },
        }

        for token in tokens {
            println!("Token: {:?}", token);
        }

        Ok(())
    }
}
