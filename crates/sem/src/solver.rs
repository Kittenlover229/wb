use crate::{
    cst::{BinopExpr, Expression, StatementBlock},
    ty::Type,
};

#[derive(Debug, Clone, Default)]
pub struct TypeSolver {
    pub id_to_ty_map: Vec<Type>,
    pub counter: u16,
}

impl TypeSolver {
    pub fn make_var_type(&mut self) -> Type {
        self.counter += 1;
        Type::Variable(self.counter)
    }

    pub fn emplace_type_variables_recursively(&mut self, expr: &mut Expression) {
        expr.ty = self.make_var_type();
        match &mut expr.expr {
            crate::cst::Expr::Binop(BinopExpr { lhs, rhs, .. }) => {
                self.emplace_type_variables_recursively(lhs);
                self.emplace_type_variables_recursively(rhs);
            }
            _ => {}
        }
    }
}

pub(crate) fn emplace_types_in_block(solver: &mut TypeSolver, block: &mut StatementBlock) {
    for stmt in &mut block.stmts {
        match &mut stmt.stmt {
            crate::cst::Stmt::NameDeclaration { value, .. } => {
                solver.emplace_type_variables_recursively(value)
            }
        }
    }
}
