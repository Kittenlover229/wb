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
        solver.emplace_type_vars_in_stmts(stmt)
    }

    for i in 1..=8 {
        let mut visitor = CstGraphvizVisualizer::default();
        visitor.visit_stmt_block(&block);
        visitor
            .dump(&mut fs::File::create(format!("out{i}.dot").as_str()).unwrap())
            .unwrap();
        println!("{i}");

        solver.solve_stmt_block_recursive(&mut block);
        for expr in solver.shallow_expr_iterator_from_stmt_block(&mut block) {
            solver.apply_constraints_recursive(expr);
        }
    }

    println!("{solver:?}");
}
