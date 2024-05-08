use tik::define_ast;
use crate::error::*;
use crate::expr::*;

define_ast!(
    Stmt/StmtVisitor,
    Expression/ExpressionStmt/visit_expression_stmt { expression: Expr }
    Print/PrintStmt/visit_print_stmt { expression: Expr }
    );
