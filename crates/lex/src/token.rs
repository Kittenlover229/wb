use std::fmt::Debug;

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

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    While,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier,
    Punctuation,
    Whitespace(u32),
    Indent,
    Dendent,
    Newline,
    Integer,
}

#[derive(Debug)]
pub struct Token {
    pub loc: SourceLocation,
    pub span: SourceSpan,
    pub kind: TokenKind,
}
