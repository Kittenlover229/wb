#[derive(Debug, Clone, Copy)]
pub struct SourceLocation;

#[derive(Debug)]
pub enum TokenKind {
    Keyword,
    Identifier,
    Punctuation,
    Whitespace,
    Integer,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub capture: &'a str,
    pub loc: SourceLocation,
    pub kind: TokenKind,
}
