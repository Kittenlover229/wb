use std::fmt::Debug;

use r#macro::SourceObject;

#[derive(Clone, Copy)]
pub struct SourceLocation {
    pub index: usize,
    pub col: u32,
    pub lineno: u32,
}

impl Debug for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SourceLocation { index, col, lineno } = self;

        f.write_str(format!("{{ idx {index} col {col} lineno {lineno} }}").as_str())
    }
}

impl SourceLocation {
    pub fn advance(&mut self, _input: &str, by: usize) {
        for ch in _input.chars().take(by) {
            if ch == '\r' {
                continue;
            } else if ch == '\n' {
                self.col = 1;
                self.lineno += 1;
            } else {
                self.col += 1;
            }
        }
        self.index += by;
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            index: 0,
            col: 1,
            lineno: 1,
        }
    }
}

pub type SourceSpan = (usize, usize);

pub trait SourceObject {
    fn source_location(&self) -> SourceLocation;
    fn source_span(&self) -> SourceSpan;
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    While,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Equals,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Greater,
    Less,
}

impl Into<&str> for Operator {
    fn into(self) -> &'static str {
        use Operator::*;

        match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Greater => ">",
            Less => "<",
            Equals => "=",
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            "%" => Ok(Operator::Mod),
            ">" => Ok(Operator::Greater),
            "<" => Ok(Operator::Less),
            "=" => Ok(Operator::Equals),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Colon,
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier(String),
    Operator(Operator),
    CompoundOperator(Operator),
    Punctuation(Punctuation),
    Whitespace(u32),
    Indent,
    Dendent,
    Newline,
    LeftParenthese,
    RightParenthese,
    Integer(String),
    End,
}

#[derive(Debug, SourceObject)]
pub struct Token {
    pub loc: SourceLocation,
    pub span: SourceSpan,
    pub kind: TokenKind,
}

impl Token {
    pub fn is_binop(&self) -> bool {
        match self.kind {
            TokenKind::Operator(_) => true,
            _ => false,
        }
    }
}
