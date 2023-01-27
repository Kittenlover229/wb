use crate::ty::Type;
use lex::Operator;

#[derive(Debug, Clone)]
pub struct StatementBlock {
    pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    NameDeclaration { name: String, value: Expression },
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub ty: Type,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Name(String),
    Binop(BinopExpr),
    Integer(String),
}

#[derive(Debug, Clone)]
pub struct BinopExpr {
    pub op: Operator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}
