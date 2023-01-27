use lex::{SourceLocation, SourceObject, SourceSpan, Operator};

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
    BinaryExpression(BinaryExpression),
}

impl SourceObject for Expression {
    fn source_location(&self) -> SourceLocation {
        match self {
            IntegerLiteral(lit) => lit.source_location(),
            BinaryExpression(binexpr) => binexpr.source_location(),
        }
    }

    fn source_span(&self) -> SourceSpan {
        match self {
            IntegerLiteral(lit) => lit.source_span(),
            BinaryExpression(binexpr) => binexpr.source_span(),
        }
    }
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

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub(crate) operator: Operator,
    pub(crate) lhs: Box<Expression>,
    pub(crate) rhs: Box<Expression>,

    pub(crate) span: SourceSpan,
    pub(crate) loc: SourceLocation,
}

impl SourceObject for BinaryExpression {
    fn source_location(&self) -> SourceLocation {
        self.loc
    }

    fn source_span(&self) -> SourceSpan {
        self.span
    }
}
