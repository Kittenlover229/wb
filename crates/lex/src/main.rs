use std::fs;

use indent::indented_tokens;
use tokenizer::try_tokenize;

mod rules;
mod token;
mod tokenizer;
mod indent;

fn main() {
    let contents = fs::read_to_string("sample.wb").expect("Should have been able to read the file");
    let tokens = try_tokenize(contents.as_str());
    let indented_toks = indented_tokens(tokens);

    for tok in indented_toks {
        println!("{tok:?}");
    }
}
