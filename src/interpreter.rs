use crate::expr::*;
use crate::token::*;
use crate::token_type::*;
use crate::error::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self,expr: &LiteralExpr) -> Result<Object, TikError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, TikError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&self,expr: &UnaryExpr) -> Result<Object, TikError> {
        let right = self.evaluate(&expr.right)?;
        
        match expr.operator.ttype {
            TokenType::Minus => match right {
                Object::Num(number) => {
                    return Ok(Object::Num(-number));
                },
                _ => { 
                    return Ok(Object::Nil);
                },
            } ,
            TokenType::Bang => if self.is_truthy(&right) {
                return Ok(Object::False);
            } else {
                return Ok(Object::True);
            },
            _ => {
                return Err(TikError::error(expr.operator.line, "Unreachable".to_string()));
            },
        }
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, TikError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Greater => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        if left > right {
                            return Ok(Object::True);
                        } else {
                            return Ok(Object::False);
                        }
                    }
                }
                return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
            },
            TokenType::GreaterEqual => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        if left >= right {
                            return Ok(Object::True);
                        } else {
                            return Ok(Object::False);
                        }
                    }
                }
                return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
            },
            TokenType::Less => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        if left < right {
                            return Ok(Object::True);
                        } else {
                            return Ok(Object::False);
                        }
                    }
                }
                return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
            },
            TokenType::LessEqual=> {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        if left <= right {
                            return Ok(Object::True);
                        } else {
                            return Ok(Object::False);
                        }
                    }
                }
                return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
            },
            TokenType::Bang => {
                Ok(Object::Nil)
            },
            TokenType::BangEqual => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        if left != right {
                            return Ok(Object::True);
                        } else {
                            return Ok(Object::False);
                        }
                    }
                }
                return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
            },
            TokenType::EqualEqual => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        if left == right {
                            return Ok(Object::True);
                        } else {
                            return Ok(Object::False);
                        }
                    }
                }
                return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
            },
            TokenType::Minus => {
                let result = left - right;
                if result == Object::ArithmeticException {
                    return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
                } else {
                    return Ok(result);
                }
            },
            TokenType::Plus => {
                let result = left + right;
                if result == Object::ArithmeticException {
                    return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
                } else {
                    return Ok(result);
                }
            },
            TokenType::Slash=> {
                let result = left / right;
                if result == Object::ArithmeticException {
                    return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
                } else {
                    return Ok(result);
                }
            },
            TokenType::Star => {
                let result = left * right;
                if result == Object::ArithmeticException {
                    return Err(TikError::error(expr.operator.line, "Invalid expression".to_string()));
                } else {
                    return Ok(result);
                }
            },
            _ => {
                return Err(TikError::error(expr.operator.line, "Unreachable".to_string()));
            },
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, TikError> {
        expr.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Nil |
            Object::False => { return false; },
            _ => { return true; },
        }
    }

    fn is_equal(lhs: &Object, rhs: &Object) -> bool {
        true
    }
}
