use std::fs;

use err::{ErrorAwareTokenStream, TokenizerResult, NonTokenizableSubstringError};
use indent::indented_tokens;
use token::Token;

mod err;
mod indent;
mod rules;
mod token;
mod tokenizer;

fn main() {
    let contents = fs::read_to_string("sample.wb").expect("Should have been able to read the file");
    let tokens: Vec<TokenizerResult> =
        ErrorAwareTokenStream::new(&contents.as_str()).collect();
    let tokens: Result<Vec<Token>, NonTokenizableSubstringError> = tokens.into_iter().collect();
    let tokens = tokens.unwrap();
    let indented_toks = indented_tokens(tokens.into_iter());

    for tok in indented_toks {
        println!("{tok:?}");
    }
}
