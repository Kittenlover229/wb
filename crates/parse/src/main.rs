mod ast;
mod parser;

use ast::*;
use lex::*;
use parser::Parser;
use std::fs;

fn main() {
    let contents = fs::read_to_string("sample.wb").expect("Should have been able to read the file");
    let tokens = try_tokenize(contents.as_str());
    let indented_toks = indented_tokens(tokens);

    for tok in &indented_toks {
        println!("{tok:?}");
    }

    let toks = indented_toks.into_iter().filter(|t|!matches!(t.kind, TokenKind::Whitespace(_)));

    let mut parser = Parser::new(toks);
    let vardecl = parser.parse_var_decl().unwrap();
    println!("{vardecl:?}")
}
