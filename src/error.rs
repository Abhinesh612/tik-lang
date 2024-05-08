use crate::token::*;
use crate::token_type::*;

#[derive(Debug)]
pub struct TikError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl TikError {
    pub fn error(line: usize, message: String) -> TikError {
        TikError { token: None, line, message }
    }

    pub fn error_parser(token: Token, message: String) -> TikError {
        let e = TikError {
            token: Some(token.clone()),
            line: token.line, 
            message };
        e.report("".to_string());
        e
    }

    pub fn report(&self, loc: String) {
        if let Some(token) = &self.token {
            if token.ttype == TokenType::Eof {
                eprintln!("{} at end {}", token.line, self.message);
            } else {
                eprintln!("{} at '{}' {}", token.line, token.lexeme, self.message);
            }
        } else {
            eprintln!("[line {}] Error {}: {}", self.line, loc, self.message);
        }
    }
}
