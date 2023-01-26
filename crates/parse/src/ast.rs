use lex::{SourceLocation, SourceSpan};

pub trait AstNode {
    fn source_location(&self) -> (SourceSpan, SourceLocation);
}

#[derive(Debug, Clone)]
pub enum Statement {
    AssignmentStmt(AssignmentStatement),
}

pub use Statement::*;

#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub identifier: String,
    pub expr: Expression,

    pub(crate) loc: SourceLocation,
    pub(crate) span: SourceSpan,
}

impl AstNode for AssignmentStatement {
    fn source_location(&self) -> (SourceSpan, SourceLocation) {
        (self.span, self.loc)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerExpr(IntegerExpression),
}

pub use Expression::*;

#[derive(Debug, Clone)]
pub struct IntegerExpression {
    pub(crate) number: String,
}
