use std::fs;

use tokenizer::try_tokenize;

mod tokenizer;
mod token;
mod rules;

fn main() {
    let contents = fs::read_to_string("sample.wb")
    .expect("Should have been able to read the file");
    let tokens = try_tokenize(contents.as_str());

    for tok in tokens {
        println!("{tok:?}");
    }
}
