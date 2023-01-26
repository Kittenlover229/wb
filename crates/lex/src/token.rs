use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct SourceLocation {
    pub index: usize,
}

impl Debug for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SourceLocation {index} = self;

        f.write_str(format!("{{ idx {index:?} }}").as_str())
    }
}

impl SourceLocation {
    pub fn advance(&mut self, _input: &str, by: usize) {
        self.index += by;
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self { index: 0 }
    }
}

pub type SourceSpan = (usize, usize);

#[derive(Debug)]
pub enum TokenKind {
    Keyword,
    Identifier,
    Punctuation,
    Whitespace,
    Integer,
}

#[derive(Debug)]
pub struct Token {
    pub loc: SourceLocation,
    pub span: SourceSpan,
    pub kind: TokenKind,
}
