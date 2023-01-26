use tokenizer::try_tokenize;

mod tokenizer;
mod token;
mod rules;

fn main() {
    let input = "let integer = 30;";
    let tokens = try_tokenize(input);

    for tok in tokens {
        println!("{tok:?}");
    }
}
