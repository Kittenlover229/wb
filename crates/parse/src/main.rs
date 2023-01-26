mod ast;
mod parser;

use ast::*;
use lex::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("sample.wb").expect("Should have been able to read the file");
    let tokens = try_tokenize(contents.as_str());
    let indented_toks = indented_tokens(tokens);

    let expected_tree = AssignmentStmt(AssignmentStatement {
        identifier: "x".to_string(),
        expr: IntegerExpr(IntegerExpression {
            number: "4".to_string(),
        }),

        loc: Default::default(),
        span: Default::default(),
    });

    for tok in indented_toks {
        println!("{tok:?}");
    }
}
