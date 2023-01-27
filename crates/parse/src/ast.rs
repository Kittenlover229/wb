use lex::{Operator, SourceLocation, SourceObject, SourceSpan};

#[derive(Debug, Clone)]
pub enum Statement {
    NameDeclStmt(NameDeclarationStatement),
}

pub use Statement::*;

impl SourceObject for Statement {
    fn source_location(&self) -> SourceLocation {
        match self {
            NameDeclStmt(stmt) => stmt.source_location(),
        }
    }

    fn source_span(&self) -> SourceSpan {
        match self {
            NameDeclStmt(stmt) => stmt.source_span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameDeclarationStatement {
    pub name: NameExpression,
    pub rhs: Expression,

    pub(crate) span: SourceSpan,
    pub(crate) loc: SourceLocation,
}

impl SourceObject for NameDeclarationStatement {
    fn source_location(&self) -> SourceLocation {
        self.loc
    }

    fn source_span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone)]
pub struct StatementBlock {
    pub(crate) loc: SourceLocation,
    pub(crate) span: SourceSpan,

    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(IntegerLiteral),
    BinaryExpression(BinaryExpression),
    NameExpression(NameExpression),
}

impl SourceObject for Expression {
    fn source_location(&self) -> SourceLocation {
        match self {
            IntegerLiteral(lit) => lit.source_location(),
            BinaryExpression(binexpr) => binexpr.source_location(),
            NameExpression(nexpr) => nexpr.source_location(),
        }
    }

    fn source_span(&self) -> SourceSpan {
        match self {
            IntegerLiteral(lit) => lit.source_span(),
            BinaryExpression(binexpr) => binexpr.source_span(),
            NameExpression(nexpr) => nexpr.source_span(),
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

#[derive(Debug, Clone)]
pub struct NameExpression {
    pub span: SourceSpan,
    pub loc: SourceLocation,

    pub(crate) identifier: String,
}

impl SourceObject for NameExpression {
    fn source_location(&self) -> SourceLocation {
        self.loc
    }

    fn source_span(&self) -> SourceSpan {
        self.span
    }
}
