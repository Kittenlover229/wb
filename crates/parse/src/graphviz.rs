use std::io::{self, Write};

use crate::ast::{Expression, Statement, StatementBlock, Expr, BinopExpr, Stmt, FunctionApplication};

#[derive(Clone, Default)]
pub struct AstGraphvizVisualizer {
    pub nodes: Vec<(i32, String)>,
    pub edges: Vec<(i32, i32, String)>,
    pub counter: i32,
}

impl AstGraphvizVisualizer {
    #[must_use]
    pub fn new_node(&mut self, label: &str) -> i32 {
        self.counter += 1;
        self.nodes.push((self.counter, label.to_owned()));
        self.counter
    }

    pub fn new_edge(&mut self, start: i32, end: i32, label: &str) {
        self.edges.push((start, end, label.to_owned()))
    }

    pub fn dump<W: Write>(&self, out: &mut W) -> io::Result<()> {
        out.write("digraph {\n".as_bytes())?;
        out.write("\trankdir=LR;\n".as_bytes())?;
        for (vert, label) in &self.nodes {
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
        match &stmt.stmt {
            Stmt::NameDeclaration { name, value } => {
                let this = self.new_node("Name Declaration");
                let name = self.new_node(name.as_str());
                let value = self.visit_expression(&value);

                self.new_edge(this, name, "name");
                self.new_edge(this, value, "value");

                this
            },
            Stmt::WhileStmt { pred, body } => {
                let this = self.new_node("While");
                let pred = self.visit_expression(pred);
                let body = self.visit_stmt_block(body);

                self.new_edge(this, pred, "pred");
                self.new_edge(this, body, "body");

                this
            },
            Stmt::Expression(expr) => self.visit_expr(expr),
        }
    }

    pub fn visit_expr(&mut self, expr: &Expr) -> i32 {
        match expr {
            Expr::Name(name) => {
                self.new_node(name.as_str())
            },
            Expr::Binop(binop) => {
                self.visit_binop(binop)
            },
            Expr::IntegerLiteral(number) => {
                self.new_node(number.as_str())
            },
            Expr::FunctionApplication(fa) => {
                self.visit_function_application(fa)
            },
            Expr::Grouping { expr } => {
                let this = self.new_node("Grouping");
                let grouped = self.visit_expression(expr);
                self.new_edge(this, grouped, "");               
                this
            }
        }
    }

    pub fn visit_expression(&mut self, expr: &Expression) -> i32 {
        self.visit_expr(&expr.expr)
    }

    pub fn visit_function_application(&mut self, fa: &FunctionApplication) -> i32 {
        let this = self.new_node("Function Application");

        let func = self.visit_expression(&fa.func);
        self.new_edge(this, func, "func");

        for (i, arg) in fa.args.iter().enumerate() {
            let arg = self.visit_expression(arg);
            self.new_edge(this, arg, (i + 1).to_string().as_str());
        }

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
