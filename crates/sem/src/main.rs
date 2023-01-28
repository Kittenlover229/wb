mod ast2cst;
mod cst;
mod graphviz;
mod solver;
mod ty;

use cst::StatementBlock;
use graphviz::CstGraphvizVisualizer;
use lex::*;
use parse::Parser;
use solver::TypeSolver;
use std::fs;

use crate::ty::Typed;

fn main() {
    let contents = fs::read_to_string("sample.wb").expect("Should have been able to read the file");
    let tokens = try_tokenize(contents.as_str());
    let indented_toks = indented_tokens(tokens);

    let toks: Vec<Token> = omitted_spaces(indented_toks).into_iter().collect();
    let mut parser = Parser::new(toks);
    let block = parser.parse_stmt_block().unwrap();
    let mut block: StatementBlock = block.into();

    let mut solver = TypeSolver::default();
    for stmt in &mut block.stmts {
        solver.emplace_type_vars_in_stmt(stmt)
    }

    let mut i = 0;
    while !block.is_complete() && i < 10 {
        let mut visitor = CstGraphvizVisualizer::default();
        visitor.visit_stmt_block(&block);
        visitor
            .dump(&mut fs::File::create(format!("out{i}.dot").as_str()).unwrap())
            .unwrap();

        solver.solve_stmt_block_recursive(&mut block);
        i += 1;
    }

    let mut visitor = CstGraphvizVisualizer::default();
    visitor.visit_stmt_block(&block);
    visitor
        .dump(&mut fs::File::create(format!("out{i}.dot").as_str()).unwrap())
        .unwrap();

    println!("{solver:?}");
}
