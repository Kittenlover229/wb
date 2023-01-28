use crate::{
    cst::{BinopExpr, Expr, Expression, StatementBlock},
    ty::Type,
};

#[derive(Debug, Clone, Default)]
pub struct TypeSolver {
    pub constraints: Vec<Type>,
    pub counter: u16,
}

impl TypeSolver {
    pub fn make_var_type(&mut self) -> Type {
        let typevar = Type::Variable(self.constraints.len() as u16 + 1);
        self.constraints.push(typevar.clone());
        typevar
    }

    pub fn emplace_type_variables(&mut self, expr: &mut Expression) {
        self.walk_expressions(expr, |this, expr| expr.ty = this.make_var_type());
    }

    pub fn contrain_literal_types(&mut self, expr: &mut Expression) {
        self.walk_expressions(expr, |this, expr| {
            match expr {
                Expression { ty: Type::Variable(n), expr: Expr::Integer(_) } => {
                    this.constraints[*n as usize - 1] = Type::Integer
                }
                _ => {}
            }
        });
    }

    pub fn apply_constraints(&mut self, expr: &mut Expression) {
        self.walk_expressions(expr, |this, expr| {
            match expr {
                Expression { ty: Type::Variable(n), .. } => {
                    expr.ty = this.constraints[*n as usize - 1].to_owned();
                }
                _ => {}
            }
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

pub(crate) fn do_for_all(solver: &mut TypeSolver, block: &mut StatementBlock, func: fn(&mut TypeSolver, &mut Expression)) {
    for stmt in &mut block.stmts {
        match &mut stmt.stmt {
            crate::cst::Stmt::NameDeclaration { value, .. } => {
                func(solver, value)
            }
        }
    }
}
