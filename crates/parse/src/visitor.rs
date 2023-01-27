use crate::ast::{
    BinaryExpression, Expression, IntegerLiteral, NameDeclarationStatement, NameExpression,
    Statement, StatementBlock,
};

pub trait Visitor<T = ()> {
    fn visit_statement(&mut self, stmt: &Statement) -> T {
        match stmt {
            Statement::NameDeclStmt(v) => self.visit_vardecl(v),
        }
    }

    fn visit_expression(&mut self, expr: &Expression) -> T {
        match expr {
            IntegerLiteral(i) => self.visit_integer_literal(i),
            BinaryExpression(b) => self.visit_binary_expr(b),
            NameExpression(name) => self.visit_name(name),
        }
    }

    fn visit_statement_block(&mut self, block: &StatementBlock) -> T;
    fn visit_name(&mut self, name: &NameExpression) -> T;
    fn visit_vardecl(&mut self, vardeclstmt: &NameDeclarationStatement) -> T;
    fn visit_integer_literal(&mut self, integer: &IntegerLiteral) -> T;
    fn visit_binary_expr(&mut self, expr: &BinaryExpression) -> T;
}
