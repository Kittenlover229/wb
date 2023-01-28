mod indent;
mod rules;
mod token;
mod tokenizer;
mod err;

pub use indent::*;
pub use rules::*;
pub use token::*;
pub use tokenizer::*;

pub fn omitted_spaces(input: impl IntoIterator<Item = Token>) -> impl IntoIterator<Item = Token> {
    input
        .into_iter()
        .filter(|token| !matches!(token.kind, TokenKind::Whitespace(_)))
}
