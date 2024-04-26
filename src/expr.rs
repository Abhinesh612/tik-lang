use tik::define_ast;
use crate::error::*;
use crate::token::*;

define_ast!(
    Expr/ExprVisitor,
    Binary/BinaryExpr/visit_binary_expr { left: Box<Expr>, operator: Token, right: Box<Expr> }
    Grouping/GroupingExpr/visit_grouping_expr  { expression: Box<Expr> }
    Literal/LiteralExpr/visit_literal_expr { value: Option<Object> }
    Unary/UnaryExpr/visit_unary_expr { operator: Token, right: Box<Expr> }
    );
