use crate::token_type::*;
use crate::token::*;
use crate::TikError;
use crate::expr::*;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, TikError> {
        self.expression()
        /*
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
        */
    }

    fn expression(&mut self) -> Result<Expr, TikError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, TikError> {
        let mut expr = self.comparision()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparision()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn comparision(&mut self) -> Result<Expr, TikError> {
        let mut expr = self.term()?;

        while self.is_match(&[TokenType::Greater, TokenType:: GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, TikError> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, TikError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, TikError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr { operator, right: Box::new(right) }));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, TikError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Object::False)}));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Object::True)}));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {value: Some(Object::Nil)}));
        }

        if self.is_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {value: self.previous().literal}))
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            let _ = self.consume(TokenType::RightParen, "Expect ')' after expression".to_string())?;
            return Ok(Expr::Grouping(GroupingExpr {expression: Box::new(expr) }));
        }

        Err(Parser::error(self.peek(), "Expect Expression".to_string()))
        //Err(TikError::error(self.peek().line, "Expect Expression".to_string()))
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, TikError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(Parser::error(self.peek(), message))
            //Err(TikError::error(p.line, message))
        }
    }

    fn synchronize(&mut self) {
        let _ = self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::SemiColon {
                return;
            }

            match self.peek().ttype {
                TokenType::Class | 
                TokenType::Fun |
                TokenType::Var |
                TokenType::For |
                TokenType::If |
                TokenType::While |
                TokenType::Print |
                TokenType::Return=> { return },
                _ => {},
            }
            let _ = self.advance();
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(*ttype) {
                self.advance();
                return true;
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
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn error(token: Token, message: String) -> TikError {
        TikError::error_parser(token, message)
    }

}
