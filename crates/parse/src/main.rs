mod ast;
mod parser;
mod visitor;
mod graphviz;

use graphviz::AstGraphvizVisualizer;
use parser::*;
use lex::*;
use visitor::Visitor;
use std::fs;

fn main() {
    let contents = fs::read_to_string("sample.wb").expect("Should have been able to read the file");
    let tokens = try_tokenize(contents.as_str());
    let indented_toks = indented_tokens(tokens);

    for tok in &indented_toks {
        println!("{tok:?}");
    }

    let toks = omitted_spaces(indented_toks);

    let mut parser = Parser::new(toks);
    let stmt = parser.parse_stmt().unwrap();

    let mut visitor = AstGraphvizVisualizer::default();
    visitor.visit_statement(&stmt);
    visitor.dump(&mut fs::File::create("out.dot").unwrap()).unwrap();

    println!("{stmt:?}")
}
