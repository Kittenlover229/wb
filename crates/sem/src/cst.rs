use crate::ty::{Type, Typed};
use lex::Operator;

#[derive(Debug, Clone)]
pub struct StatementBlock {
    pub stmts: Vec<Statement>,
}

impl Typed for StatementBlock {
    fn is_complete(&self) -> bool {
        self.stmts.iter().all(Typed::is_complete)
    }
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub stmt: Stmt,
}

impl Typed for Statement {
    fn is_complete(&self) -> bool {
        self.stmt.is_complete()
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    NameDeclaration { name: String, value: Expression },
    While { pred: Expression, body: StatementBlock },
    Expression(Expression),
}

impl Typed for Stmt {
    fn is_complete(&self) -> bool {
        match self {
            Stmt::NameDeclaration { value, .. } => value.is_complete(),
            Stmt::While { pred, body } => pred.is_complete() && body.is_complete(),
            Stmt::Expression(expr) => expr.is_complete(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub ty: Type,
    pub expr: Expr,
}

impl Typed for Expression {
    fn is_complete(&self) -> bool {
        self.ty.is_complete()
    }
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
