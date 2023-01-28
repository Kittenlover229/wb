use lex::{Operator, SourceLocation, SourceObject, SourceSpan};

#[derive(Debug, Clone, SourceObject)]
pub struct Statement {
    pub(crate) loc: SourceLocation,
    pub(crate) span: SourceSpan,

    pub stmt: Stmt,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    NameDeclaration { name: String, value: Expression },
    WhileStmt { pred: Expression, body: StatementBlock },
    Expression(Expr),
}

#[derive(Debug, Clone, SourceObject)]
pub struct StatementBlock {
    pub(crate) loc: SourceLocation,
    pub(crate) span: SourceSpan,

    pub stmts: Vec<Statement>,
}


#[derive(Debug, Clone, SourceObject)]
pub struct Expression {
    pub(crate) loc: SourceLocation,
    pub(crate) span: SourceSpan,

    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    IntegerLiteral(String),
    Binop(BinopExpr),
    Name(String),
    FunctionApplication(FunctionApplication),
    Grouping { expr: Box<Expression> },
}

use r#macro::SourceObject;

#[derive(Debug, Clone)]
pub struct BinopExpr {
    pub op: Operator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct FunctionApplication {
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
}
