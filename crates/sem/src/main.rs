mod cst;
mod ty;
mod ast2cst;
mod graphviz;
mod solver;

use cst::StatementBlock;
use graphviz::CstGraphvizVisualizer;
use parse::Parser;
use lex::*;
use solver::{TypeSolver, emplace_types_in_block};
use std::fs;

fn main() {
    let contents = fs::read_to_string("sample.wb").expect("Should have been able to read the file");
    let tokens = try_tokenize(contents.as_str());
    let indented_toks = indented_tokens(tokens);

    let toks: Vec<Token> = omitted_spaces(indented_toks).into_iter().collect();
    for (i, tok) in toks.iter().enumerate() {
        println!("{i}: {tok:?}");
    }
    
    let mut parser = Parser::new(toks);
    let block = parser.parse_stmt_block().unwrap();
    let mut block: StatementBlock = block.into();

    let mut solver = TypeSolver::default();
    emplace_types_in_block(&mut solver, &mut block);

    let mut visitor = CstGraphvizVisualizer::default();
    visitor.visit_stmt_block(&block.into());
    visitor.dump(&mut fs::File::create("out.dot").unwrap()).unwrap();
}
