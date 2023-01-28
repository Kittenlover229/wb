use std::io::{self, Write};

use crate::{
    cst::{BinopExpr, Expr, Expression, Statement, StatementBlock},
    ty::Type,
};

#[derive(Clone, Debug, Default)]
pub struct CstGraphvizVisualizer {
    pub nodes: Vec<(i32, String)>,
    pub type_nodes: Vec<(i32, String)>,
    pub edges: Vec<(i32, i32, String)>,
    pub counter: i32,
}

impl CstGraphvizVisualizer {
    #[must_use]
    pub fn new_node(&mut self, label: &str) -> i32 {
        self.counter += 1;
        self.nodes.push((self.counter, label.to_owned()));
        self.counter
    }

    pub fn new_type_node(&mut self, typename: &str) -> i32 {
        self.counter += 1;
        self.type_nodes.push((self.counter, typename.to_owned()));
        self.counter
    }

    #[must_use]
    pub fn get_type_node(&mut self, ty: &Type) -> i32 {
        match ty {
            Type::Variable(var) => self.new_type_node(format!("T{}", var).as_str()),
            Type::Integer => self.new_type_node("Integer"),
            Type::Bool => self.new_type_node("Bool"),
        }
    }

    pub fn new_edge(&mut self, start: i32, end: i32, label: &str) {
        self.edges.push((start, end, label.to_owned()))
    }

    pub fn dump<W: Write>(&self, out: &mut W) -> io::Result<()> {
        out.write("digraph {\n".as_bytes())?;
        for (vert, label) in &self.nodes {
            out.write(format!("\t{vert} [label=\"{label}\"]\n").as_bytes())?;
        }

        for (vert, label) in &self.type_nodes {
            out.write(format!("\t{vert} [label=\"{label}\" shape=none color=gray fontcolor=gray]\n").as_bytes())?;
        }

        for (start, end, label) in &self.edges {
            if self.type_nodes.iter().filter(|x| *end == x.0).count() == 1 {
                out.write(format!("\t{start}->{end} [arrowhead=onormal color=gray fontcolor=gray]\n").as_bytes())?;
            } else {
                out.write(format!("\t{start}->{end} [label=\"{label}\"]\n").as_bytes())?;
            }
        }

        out.write("}\n".as_bytes())?;
        Ok(())
    }

    pub fn visit_stmt_block(&mut self, block: &StatementBlock) -> i32 {
        let this = self.new_node("Block");
        for (i, stmt) in block.stmts.iter().enumerate() {
            let stmt = self.visit_stmt(stmt);
            self.new_edge(this, stmt, (i + 1).to_string().as_str());
        }

        this
    }

    pub fn visit_stmt(&mut self, stmt: &Statement) -> i32 {
        match &stmt.stmt {
            crate::cst::Stmt::NameDeclaration { name, value } => {
                let this = self.new_node("Name Declaration");
                let name = self.new_node(name.as_str());
                let value = self.visit_expression(&value);

                self.new_edge(this, name, "name");
                self.new_edge(this, value, "value");

                this
            }
            crate::cst::Stmt::While { pred, body } => {
                let this = self.new_node("While");
                let pred = self.visit_expression(pred);
                let body = self.visit_stmt_block(body);

                self.new_edge(this, pred, "pred");
                self.new_edge(this, body, "body");

                this
            }
            crate::cst::Stmt::Expression(expr) => todo!(),
        }
    }

    pub fn visit_expression(&mut self, expr: &Expression) -> i32 {
        let this = match &expr.expr {
            Expr::Name(name) => self.new_node(name.as_str()),
            Expr::Binop(binop) => self.visit_binop(&binop),
            Expr::Integer(number) => self.new_node(number.as_str()),
        };

        let ty = self.get_type_node(&expr.ty);
        self.new_edge(this, ty, "  : type");

        this
    }

    pub fn visit_binop(&mut self, binop: &BinopExpr) -> i32 {
        let this = self.new_node(binop.op.into());
        let lhs = self.visit_expression(&binop.lhs);
        let rhs = self.visit_expression(&binop.rhs);

        self.new_edge(this, lhs, "lhs");
        self.new_edge(this, rhs, "rhs");

        this
    }
}
