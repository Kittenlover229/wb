use std::io::{Write, self};

use crate::{cst::{StatementBlock, Statement, Expr, Expression, BinopExpr}, ty::Type};

#[derive(Clone, Debug, Default)]
pub struct CstGraphvizVisualizer {
    pub verts: Vec<(i32, String)>,
    pub ty_verts: Vec<(i32, String)>,
    pub edges: Vec<(i32, i32, String)>,
    pub counter: i32,
}


impl CstGraphvizVisualizer {
    #[must_use]
    pub fn new_node(&mut self, label: &str) -> i32 {
        self.counter += 1;
        self.verts.push((self.counter, label.to_owned()));
        self.counter
    }

    #[must_use]
    pub fn get_type_node(&mut self, ty: &Type) -> i32 {
        self.counter += 1;
        self.verts.push((self.counter, "T".to_string()));
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

    pub fn visit_stmt_block(&mut self, block: &StatementBlock) -> i32 {
        let this = self.new_node("Block");
        for (i, stmt) in block.stmts.iter().enumerate() {
            let stmt = self.visit_stmt(stmt);
            self.new_edge(this, stmt, (i + 1).to_string().as_str());
        }

        this
    }

    pub fn visit_stmt(&mut self, stmt: &Statement) -> i32 {
        match &stmt.kind {
            crate::cst::StatementKind::NameDeclaration { name, value } => {
                let this = self.new_node("Name Declaration");
                let name = self.new_node(name.as_str());
                let value = self.visit_expression(&value);

                self.new_edge(this, name, "name");
                self.new_edge(this, value, "value");

                this
            },
        }
    }

    pub fn visit_expression(&mut self, expr: &Expression) -> i32 {
        let this = match &expr.expr {
            Expr::Name(name) => {
                self.new_node(name.as_str())
            },
            Expr::Binop(binop) => {
                self.visit_binop(&binop)
            },
            Expr::Integer(number) => {
                self.new_node(number.as_str())
            },
        };

        let ty = self.visit_type(&expr.ty);
        self.new_edge(this, ty, "type");

        this
    }

    pub fn visit_type(&mut self, ty: &Type) -> i32 {
        match ty {
            Type::Variable(var) => self.new_node(format!("t{}", var).as_str()),
            Type::Integer => self.new_node("Integer"),
            Type::Bool => self.new_node("Bool"),
        }
    }

    pub fn visit_binop(&mut self, binop: &BinopExpr) -> i32 {
        let this = self.new_node(binop.op.into());
        let lhs = self.visit_expression(&binop.lhs);
        let rhs = self.visit_expression(&binop.lhs);

        self.new_edge(this, lhs, "lhs");
        self.new_edge(this, rhs, "rhs");

        this
    }
}
