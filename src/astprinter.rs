use crate::TikError;
use crate::expr::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, TikError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &[&Box<Expr>]) -> Result<String, TikError> {
        let mut builder = format!("({name}");

        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self).unwrap());
        }

        builder = format!("{builder})");
        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, TikError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, TikError> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, TikError> {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, TikError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}
