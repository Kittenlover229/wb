use std::io::{Write, self};

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
    fn visit_vardecl(&mut self, vardeclstmt: &crate::ast::VarDeclStatement) -> i32 {
        let this = self.new_node("Variable Declaration");
        let varname = self.new_node(&vardeclstmt.varname);
        let rhs = self.visit_expression(&vardeclstmt.rhs);

        self.new_edge(this, varname, "name");
        self.new_edge(this, rhs, "rhs");

        this
    }

    fn visit_integer_literal(&mut self, integer: &crate::ast::IntegerLiteral) -> i32 {
        self.new_node(&integer.number)
    }
}
