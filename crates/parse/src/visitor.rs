use crate::ast::{Expression, IntegerLiteral, Statement, VarDeclStatement, BinaryExpression};

pub trait Visitor<T = ()> {
    fn visit_statement(&mut self, stmt: &Statement) -> T {
        match stmt {
            Statement::VarDeclStmt(v) => self.visit_vardecl(v),
        }
    }

    fn visit_expression(&mut self, expr: &Expression) -> T {
        match expr {
            IntegerLiteral(i) => self.visit_integer_literal(i),
            BinaryExpression(b) => self.visit_binary_expr(b),
        }
    }

    fn visit_vardecl(&mut self, vardeclstmt: &VarDeclStatement) -> T;
    fn visit_integer_literal(&mut self, integer: &IntegerLiteral) -> T;
    fn visit_binary_expr(&mut self, expr: &BinaryExpression) -> T;
}
