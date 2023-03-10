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
            ast::Stmt::WhileStmt { pred, body } => Self::While {
                pred: pred.into(),
                body: body.into(),
            },
            ast::Stmt::Expression(expr) => Self::Expression(cst::Expression {
                expr: expr.into(),
                ty: Type::default(),
            }),
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
            ty: Type::default(),
            expr: value.expr.into(),
        }
    }
}

impl From<ast::Expr> for cst::Expr {
    fn from(value: ast::Expr) -> Self {
        match value {
            ast::Expr::IntegerLiteral(n) => Self::Integer(n),
            ast::Expr::Binop(binop) => Self::Binop {
                op: binop.op,
                lhs: Box::new((*binop.lhs).into()),
                rhs: Box::new((*binop.rhs).into()),
            },
            ast::Expr::Name(name) => Self::Name(name),
            _ => todo!(),
        }
    }
}
