use std::collections::BTreeMap;

use crate::{
    cst::{BinopExpr, Expr, Expression, Statement, StatementBlock},
    ty::Type,
};

#[derive(Debug, Clone, Default)]
pub struct TypeSolver {
    pub constraints: BTreeMap<u128, Type>,
    pub symbol_table: BTreeMap<String, Type>,
    pub counter: u128,
}

impl TypeSolver {
    pub fn make_var_type(&mut self) -> Type {
        self.counter += 1;
        let typevar = Type::Variable(self.counter);
        self.constraints.insert(self.counter, typevar.to_owned());
        typevar
    }

    pub fn emplace_type_vars_in_exprs(&mut self, expr: &mut Expression) {
        expr.ty = match &mut expr.expr {
            Expr::Name(name) => {
                let t = self.make_var_type();
                self.symbol_table.insert(name.to_owned(), t.clone());
                t
            }
            Expr::Binop(BinopExpr { lhs, rhs, .. }) => {
                self.emplace_type_vars_in_exprs(lhs);
                self.emplace_type_vars_in_exprs(rhs);
                self.make_var_type()
            }
            Expr::Integer(_) => Type::Integer,
        }
    }

    pub fn emplace_type_vars_in_stmt(&mut self, stmt: &mut Statement) {
        for expr in self.shallow_expr_iterator_from_stmt(stmt) {
            self.emplace_type_vars_in_exprs(expr);
        }
    }

    pub fn solve_stmt_block_recursive(&mut self, block: &mut StatementBlock) {
        for stmt in &mut block.stmts {
            self.solve_stmt_recursive(stmt);
        }
    }

    pub fn solve_stmt_recursive(&mut self, stmt: &mut Statement) {
        match &mut stmt.stmt {
            crate::cst::Stmt::NameDeclaration { name, value } => {
                self.symbol_table
                    .get_mut(name)
                    .map(|ty| *ty = value.ty.to_owned());
                self.solve_expr_recursive(value);
            }
            crate::cst::Stmt::While { pred, body } => {
                self.solve_expr_recursive(pred);
                self.solve_stmt_block_recursive(body);
            }
            crate::cst::Stmt::Expression(expr) => {
                self.solve_expr_recursive(expr);
            }
        }
    }

    pub fn solve_expr_recursive(&mut self, e: &mut Expression) {
        use Expr::*;
        match e {
            Expression {
                ty: Type::Variable(n),
                expr,
            } => match expr {
                Binop(BinopExpr { lhs, rhs, .. }) => {
                    self.solve_expr_recursive(lhs);
                    self.solve_expr_recursive(rhs);
                    if (*lhs).ty == (*rhs).ty {
                        self.constraints.insert(*n, lhs.ty.to_owned());
                        e.ty = lhs.ty.to_owned();
                    }
                }
                Name(name) => {
                    let ty = self.symbol_table.get(name).unwrap().to_owned();
                    self.constraints.insert(*n, ty.to_owned());
                    e.ty = ty;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn shallow_expr_iterator_from_stmt_block<'a>(
        &mut self,
        block: &'a mut StatementBlock,
    ) -> impl IntoIterator<Item = &'a mut Expression> {
        let mut out = vec![];
        for stmt in &mut block.stmts {
            out.extend(self.shallow_expr_iterator_from_stmt(stmt));
        }
        out.into_iter()
    }

    pub fn shallow_expr_iterator_from_stmt<'a>(
        &mut self,
        stmt: &'a mut Statement,
    ) -> impl IntoIterator<Item = &'a mut Expression> {
        match &mut stmt.stmt {
            crate::cst::Stmt::NameDeclaration { value, .. } => vec![value].into_iter(),
            crate::cst::Stmt::Expression(expr) => vec![expr].into_iter(),
            crate::cst::Stmt::While { pred, body } => {
                let mut out = vec![pred];
                out.extend(self.shallow_expr_iterator_from_stmt_block(body));
                out.into_iter()
            }
        }
    }
}
