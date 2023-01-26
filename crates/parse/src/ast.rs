use lex::{SourceLocation, SourceSpan, SourceObject};

pub trait AstNode {
    fn source_location(&self) -> (SourceSpan, SourceLocation);
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDeclStmt(VarDeclStatement),
}

pub use Statement::*;

#[derive(Debug, Clone)]
pub struct VarDeclStatement {
    pub varname: String,
    pub rhs: Expression,

    pub(crate) span: SourceSpan,
    pub(crate) loc: SourceLocation,
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(IntegerLiteral),
}

pub use Expression::*;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub span: SourceSpan,
    pub loc: SourceLocation,

    pub(crate) number: String,
}

impl SourceObject for IntegerLiteral {
    fn source_location(&self) -> SourceLocation {
        self.loc
    }

    fn source_span(&self) -> SourceSpan {
        self.span
    }
}
