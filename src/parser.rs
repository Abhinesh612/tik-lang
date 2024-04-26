use crate::token_type::*;
use crate::token::*;
use crate::BinaryExpr;
use crate::Expr;
use crate::TikError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr, TikError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, TikError> {
        let mut expr = self.comparision()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.pervious();
            let right = self.comparision();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparision(&mut self) -> Result<Expr, TikError> {
        let mut expr = self.term();

        while (is_match)
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for type in &types {
            if self.check(type) {
                self.advance();
                true
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ttype == ttype
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1; 
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1)
    }

}
