use crate::{cst, ty::Type};
use ast::{BinaryExpression, IntegerLiteral};
use parse as ast;

impl From<ast::StatementBlock> for cst::StatementBlock {
    fn from(value: ast::StatementBlock) -> Self {
        cst::StatementBlock {
            stmts: value.statements.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ast::Statement> for cst::Statement {
    fn from(value: ast::Statement) -> Self {
        cst::Statement { kind: value.into() }
    }
}

impl From<ast::Statement> for cst::StatementKind {
    fn from(value: ast::Statement) -> Self {
        match value {
            ast::Statement::NameDeclStmt(x) => Self::NameDeclaration {
                name: "varname".to_owned(),
                value: x.rhs.into(),
            },
            ast::Statement::WhileStmt(_) => todo!(),
            ast::Statement::ExpressionStmt(_) => todo!(),
        }
    }
}

impl From<ast::Expression> for cst::Expression {
    fn from(value: ast::Expression) -> Self {
        cst::Expression {
            ty: Type::Variable(0),
            expr: value.into(),
        }
    }
}

impl From<ast::Expression> for cst::Expr {
    fn from(value: ast::Expression) -> Self {
        match value {
            ast::Expression::IntegerLiteral(IntegerLiteral { number, .. }) => {
                cst::Expr::Integer(number)
            }
            ast::Expression::BinaryExpression(BinaryExpression {
                operator: op,
                lhs,
                rhs,
                ..
            }) => cst::Expr::Binop(cst::BinopExpr {
                op,
                lhs: Box::new((*lhs).into()),
                rhs: Box::new((*rhs).into()),
            }),
            ast::Expression::NameExpression(ast::NameExpression { identifier, .. }) => {
                cst::Expr::Name(identifier)
            }

            _ => unreachable!(),
        }
    }
}
