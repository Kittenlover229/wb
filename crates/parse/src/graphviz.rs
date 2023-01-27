use std::io::{self, Write};

use crate::visitor::Visitor;

#[derive(Clone, Default)]
pub struct AstGraphvizVisualizer {
    pub verts: Vec<(i32, String)>,
    pub edges: Vec<(i32, i32, String)>,
    pub counter: i32,
}

impl AstGraphvizVisualizer {
    #[must_use]
    pub fn new_node(&mut self, label: &str) -> i32 {
        self.counter += 1;
        self.verts.push((self.counter, label.to_owned()));
        self.counter
    }

    pub fn new_edge(&mut self, start: i32, end: i32, label: &str) {
        self.edges.push((start, end, label.to_owned()))
    }

    pub fn dump<W: Write>(&self, out: &mut W) -> io::Result<()> {
        out.write("digraph {\n".as_bytes())?;
        out.write("\trankdir=LR;\n".as_bytes())?;
        for (vert, label) in &self.verts {
            out.write(format!("\t{vert} [label=\"{label}\"]\n").as_bytes())?;
        }
        for (start, end, label) in &self.edges {
            out.write(format!("\t{start}->{end} [label=\"{label}\"]\n").as_bytes())?;
        }
        out.write("}\n".as_bytes())?;
        Ok(())
    }
}

impl Visitor<i32> for AstGraphvizVisualizer {
    fn visit_vardecl(&mut self, vardeclstmt: &crate::ast::NameDeclarationStatement) -> i32 {
        let this = self.new_node("Variable Declaration");
        let varname = self.visit_name(&vardeclstmt.name);
        let rhs = self.visit_expression(&vardeclstmt.rhs);

        self.new_edge(this, varname, "name");
        self.new_edge(this, rhs, "rhs");

        this
    }

    fn visit_integer_literal(&mut self, integer: &crate::ast::IntegerLiteral) -> i32 {
        self.new_node(&integer.number)
    }

    fn visit_binary_expr(&mut self, expr: &crate::ast::BinaryExpression) -> i32 {
        let this = self.new_node(expr.operator.into());
        let lhs = self.visit_expression(&expr.lhs);
        let rhs = self.visit_expression(&expr.rhs);

        self.new_edge(this, lhs, "lhs");
        self.new_edge(this, rhs, "rhs");

        this
    }

    fn visit_name(&mut self, name: &crate::ast::NameExpression) -> i32 {
        self.new_node(&name.identifier)
    }

    fn visit_statement_block(&mut self, block: &crate::ast::StatementBlock) -> i32 {
        let this = self.new_node("Block");

        for (i, stmt) in block.statements.iter().enumerate() {
            let stmt = self.visit_statement(stmt);
            self.new_edge(this, stmt, (i + 1).to_string().as_str());
        }

        this
    }

    fn visit_while(&mut self, w: &crate::ast::WhileStatement) -> i32 {
        let this = self.new_node("While");
        let pred = self.visit_expression(&w.pred);
        let body = self.visit_statement_block(&w.body);

        self.new_edge(this, pred, "pred");
        self.new_edge(this, body, "body");

        this
    }

    fn visit_function_application(&mut self, f: &crate::ast::FunctionApplication) -> i32 {
        let this = self.new_node("Function Applicatcion");
        let func = self.visit_expression(f.func.as_ref());
        self.new_edge(this, func, "func");
        for (i, arg) in f.args.iter().enumerate() {
            let arg = self.visit_expression(arg);
            self.new_edge(this, arg, (i + 1).to_string().as_str());
        }
        this
    }

    fn visit_grouping(&mut self, g: &crate::ast::Grouping) -> i32 {
        let this = self.new_node("Grouping");
        let expr = self.visit_expression(&g.expr);
        self.new_edge(this, expr, "grouped");
        this
    }
}
