use crate::{cst, ty::Type};
use parse as ast;

impl From<ast::Statement> for cst::Statement {
    fn from(value: ast::Statement) -> Self {
        Self {
            stmt: value.stmt.into(),
        }
    }
}

impl From<ast::Stmt> for cst::Stmt {
    fn from(value: ast::Stmt) -> Self {
        match value {
            ast::Stmt::NameDeclaration { name, value } => Self::NameDeclaration {
                name,
                value: value.into(),
            },
            _ => todo!(),
        }
    }
}

impl From<ast::StatementBlock> for cst::StatementBlock {
    fn from(value: ast::StatementBlock) -> Self {
        Self {
            stmts: value.stmts.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ast::Expression> for cst::Expression {
    fn from(value: ast::Expression) -> Self {
        Self {
            ty: Type::Variable(0),
            expr: value.expr.into(),
        }
    }
}

impl From<ast::Expr> for cst::Expr {
    fn from(value: ast::Expr) -> Self {
        match value {
            ast::Expr::IntegerLiteral(n) => Self::Integer(n),
            ast::Expr::Binop(binop) => Self::Binop(binop.into()),
            ast::Expr::Name(name) => Self::Name(name),
            _ => todo!(),
        }
    }
}

impl From<ast::BinopExpr> for cst::BinopExpr {
    fn from(value: ast::BinopExpr) -> Self {
        Self {
            op: value.op,
            lhs: Box::new((*value.lhs).into()),
            rhs: Box::new((*value.rhs).into())
        }
    }
}
