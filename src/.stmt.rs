use crate::error::*;
use crate::token::*;
use crate::expr::*;


pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, TikError> {
        match self {
            Stmt::Expression(x) => x.accept(stmt_visitor),
            Stmt::Print(x) => x.accept(stmt_visitor),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

impl ExpressionStmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, TikError> {
        stmt_visitor.visit_expression_stmt(self)
    }
}

pub struct PrintStmt {
    pub expression: Expr,
}

impl PrintStmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, TikError> {
        stmt_visitor.visit_print_stmt(self)
    }
}

