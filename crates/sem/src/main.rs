mod ast2cst;
mod cst;
mod graphviz;
mod solver;
mod ty;

use cst::StatementBlock;
use graphviz::CstGraphvizVisualizer;
use lex::*;
use parse::Parser;
use solver::{do_for_all_exprs, TypeSolver};
use std::fs;

use crate::solver::do_for_all_stmts;

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
    do_for_all_exprs(&mut solver, &mut block, TypeSolver::emplace_type_vars_in_exprs);
    do_for_all_stmts(&mut solver, &mut block, TypeSolver::emplace_type_vars_in_stmts);
    do_for_all_exprs(&mut solver, &mut block, TypeSolver::constrain_literal_types);
    println!("{solver:?}");

    for i in 1..=8 {
        let mut visitor = CstGraphvizVisualizer::default();
        visitor.visit_stmt_block(&block);
        visitor
            .dump(&mut fs::File::create(format!("out{i}.dot").as_str()).unwrap())
            .unwrap();

        do_for_all_stmts(&mut solver, &mut block, TypeSolver::solve_name_decls);
        do_for_all_exprs(&mut solver, &mut block, TypeSolver::solve_exprs);
        do_for_all_exprs(&mut solver, &mut block, TypeSolver::apply_constraints);
    }

    println!("{solver:?}");
}
