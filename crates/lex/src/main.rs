use tokenizer::tokenize;

mod tokenizer;
mod token;

fn main() {
    let input = "let integer = 30;";
    let tokens = tokenize(input);

    for tok in tokens {
        println!("{tok:?}");
    }
}
