use crate::token::*;
use crate::token_type::*;
use crate::error::*;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, TikError> {
        let mut had_error: Option<TikError> = None;
        
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {},
                Err(e) => {
                    e.report("Unexpected character".to_string());
                    had_error = Some(e);
                }
            }
        }

        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), TikError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Coma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok);
            },
            '=' => {
                let tok = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tok);
            },
            '<' => {
                let tok = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok);
            },
            '>' => {
                let tok = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok);
            },
            '/' => {

                if self.is_match('/') {
                    // A comment goes until the end of line
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else if self.is_match('*'){
                    // Block Comment
                    while let Some(_) = self.peek() {
                        if self.is_match('*') {
                            if self.is_match('/') {
                                break;
                            } 
                        } else if self.is_match('\n') {
                            self.line += 1;
                        }
                        else {
                            self.advance();
                        }
                    }
                }
                else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => { self.line += 1; },
            '"' => {
                self.string()?;
            },
            '0'..='9' =>  {
                self.number()?;
            }
            _  if c.is_ascii_alphabetic() || c == '_' => {
                    self.identifier();
                },
            
            _   => {
                return Err(TikError::error(self.line, "Unexpected charater".to_string()));
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let result = self.source.get(self.current).unwrap();
        self.current += 1;
        *result
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
        let lexeme: String = self.source[self.start .. self.current].iter().collect();
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line));
    }

    fn string(&mut self) -> Result<(), TikError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    break;
                },
                '\n' => {
                    self.line += 1;
                },
                _ => {}
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(TikError::error(self.line, "Unterminated string".to_string()));
        }

        self.advance();

        let value = self.source[self.start + 1 .. self.current -1].iter().collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }

    fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphanumeric()
        } else {
            false
        }
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn number(&mut self) -> Result<(), TikError> {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if let Some(ch) = self.peek() {
            if ch == '.' {
                if Scanner::is_digit(self.peek_next()) {
                    self.advance();

                    while Scanner::is_digit(self.peek()) {
                        self.advance();
                    }
                }
            }
        }

        let value: String = self.source[self.start .. self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(num)));
        Ok(())
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start .. self.current].iter().collect();
        if let Some(ttype) = Scanner::keyword(text.as_str()) {
            self.add_token(ttype);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }

            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "and"    => Some(TokenType::And),
            "class"  => Some(TokenType::Class),
            "else"   => Some(TokenType::Else),
            "false"  => Some(TokenType::False),
            "fun"    => Some(TokenType::Fun),
            "if"     => Some(TokenType::If),
            "nil"    => Some(TokenType::Nil),
            "or"     => Some(TokenType::Or), 
            "print"  => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super"  => Some(TokenType::Super), 
            "this"   => Some(TokenType::This), 
            "true"   => Some(TokenType::True),
            "var"    => Some(TokenType::Var),
            "while"  => Some(TokenType::While),
            "for"    => Some(TokenType::For),
            _ => None,
        }
    }
}
