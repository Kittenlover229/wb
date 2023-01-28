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
        self.walk_expressions(expr, |this, expr| expr.ty = this.make_var_type());
    }

    pub fn emplace_type_vars_in_stmts(&mut self, stmt: &mut Statement) {
        match &mut stmt.stmt {
            crate::cst::Stmt::NameDeclaration {
                name,
                value:
                    Expression {
                        ty: Type::Variable(n),
                        ..
                    },
            } if *n != 0 => {
                self.symbol_table
                    .insert(name.to_owned(), Type::Variable(*n));
            }
            _ => {}
        }
    }

    pub fn constrain_literal_types(&mut self, expr: &mut Expression) {
        self.walk_expressions(expr, |this, expr| match expr {
            Expression {
                ty: Type::Variable(n),
                expr: Expr::Integer(_),
            } => *this.constraints.get_mut(n).unwrap() = Type::Integer,
            _ => {}
        });
    }

    pub fn solve_stmts(&mut self, stmt: &mut Statement) {
        match &mut stmt.stmt {
            crate::cst::Stmt::NameDeclaration { name, value } => {
                self.symbol_table
                    .get_mut(name)
                    .map(|ty| *ty = value.ty.to_owned());
            }
            crate::cst::Stmt::While { pred, body } => {
                self.solve_exprs(pred);
                for stmt in &mut body.stmts {
                    self.solve_stmts(stmt);
                }
            }
            _ => {}
        }
    }

    // Temporary function since inary operations are not defined in the symbol table just yet
    pub fn solve_exprs(&mut self, expr: &mut Expression) {
        self.walk_expressions(expr, |this, expr| {
            use Expr::*;
            match expr {
                Expression {
                    ty: Type::Variable(n),
                    expr,
                } => match expr {
                    Binop(BinopExpr { lhs, rhs, .. }) => {
                        if (*lhs).ty == (*rhs).ty {
                            this.constraints.insert(*n, lhs.ty.to_owned());
                        }
                        this.solve_exprs(lhs);
                        this.solve_exprs(rhs);
                    }
                    Name(name) => {
                        this.constraints
                            .insert(*n, this.symbol_table.get(name).unwrap().to_owned());
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }

    pub fn apply_constraints(&mut self, expr: &mut Expression) {
        self.walk_expressions(expr, |this, expr| match expr {
            Expression {
                ty: Type::Variable(n),
                ..
            } => {
                expr.ty = this.constraints.get(n).unwrap().to_owned();
            }
            _ => {}
        });
    }

    pub fn walk_expressions(
        &mut self,
        target: &mut Expression,
        func: fn(&mut Self, &mut Expression),
    ) {
        func(self, target);
        match &mut target.expr {
            crate::cst::Expr::Binop(BinopExpr { lhs, rhs, .. }) => {
                self.walk_expressions(lhs, func);
                self.walk_expressions(rhs, func);
            }
            _ => {}
        }
    }
}

pub(crate) fn do_for_all_exprs(
    solver: &mut TypeSolver,
    block: &mut StatementBlock,
    func: fn(&mut TypeSolver, &mut Expression),
) {
    for stmt in &mut block.stmts {
        match &mut stmt.stmt {
            crate::cst::Stmt::NameDeclaration { value, .. } => func(solver, value),
            crate::cst::Stmt::While { pred, body } => {
                func(solver, pred);
                do_for_all_exprs(solver, body, func);
            }
            crate::cst::Stmt::Expression(_) => todo!(),
        }
    }
}

pub(crate) fn do_for_all_stmts(
    solver: &mut TypeSolver,
    block: &mut StatementBlock,
    func: fn(&mut TypeSolver, &mut Statement),
) {
    for stmt in &mut block.stmts {
        func(solver, stmt);
    }
}
