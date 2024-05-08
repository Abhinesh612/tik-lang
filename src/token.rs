use crate::token_type::*;
use std::fmt;
use std::ops::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
    ArithmeticException,
}

impl Sub for Object {
    type Output = Object;
    fn sub(self, rhs: Self) -> Self::Output {
       match (self, rhs) {
           (Object::Num(left), Object::Num(right)) => {Object::Num(left - right)},
           _ => {Object::ArithmeticException},
       }
    }
}

impl Div for Object {
    type Output = Object;
    fn div(self, rhs: Self) -> Self::Output {
       match (self, rhs) {
           (Object::Num(left), Object::Num(right)) => {Object::Num(left / right)},
           _ => {Object::ArithmeticException},
       }
    }
}

impl Mul for Object {
    type Output = Object;
    fn mul(self, rhs: Self) -> Self::Output {
       match (self, rhs) {
           (Object::Num(left), Object::Num(right)) => {Object::Num(left * right)},
           _ => {Object::ArithmeticException},
       }
    }
}

impl Add for Object {
    type Output = Object;
    fn add(self, rhs: Self) -> Self::Output {
       match (self, rhs) {
           (Object::Num(left), Object::Num(right)) => {Object::Num(left + right)},
           (Object::Str(left), Object::Str(right)) => {Object::Str(format!("{left}{right}"))},
           _ => {Object::ArithmeticException},
       }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match(self, other) {
            (Object::Num(left), Object::Num(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil    => write!(f, "nil"),
            Object::True   => write!(f, "true"),
            Object::False  => write!(f, "false"),
            Object::ArithmeticException => panic!("Unreachable for printing"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token {ttype, lexeme, literal, line }
    }

    pub fn eof(line: usize) -> Token {
        Token { ttype: TokenType::Eof, lexeme: "".to_string(), literal: None, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {} {}", self.ttype, self.lexeme,
               if let Some(literal) = &self.literal {
                   literal.to_string()
               } else {
                   "None".to_string()
               }, self.line)
    }
}
