use lex::{Keyword, Operator, Punctuation, SourceLocation, SourceObject, Token, TokenKind};

use crate::ast::{
    BinopExpr, Expr, Expression, FunctionApplication, Statement, StatementBlock, Stmt,
};

pub struct Parser {
    cursor: usize,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
pub struct ParserFault {
    pub loc: SourceLocation,
}

pub type ParserResult<T> = Result<T, ParserFault>;

pub fn precedence_of(op: &Operator) -> u8 {
    use Operator::*;

    match op {
        Mul | Div | Mod => 5,
        Add | Sub => 6,
        Greater | Less => 9,
        Equals => 10,
    }
}

impl Parser {
    pub fn new(tokens: impl IntoIterator<Item = Token>) -> Self {
        Self {
            cursor: 0,
            tokens: tokens.into_iter().collect(),
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn eat_if(&mut self, pred: impl FnOnce(&Token) -> bool) -> ParserResult<&Token> {
        let cur = &self.tokens[self.cursor];
        if pred(cur) {
            self.cursor += 1;
            Ok(cur)
        } else {
            Err(ParserFault {
                loc: cur.source_location(),
            })
        }
    }

    fn eat<T>(&mut self, map: impl FnOnce(&Token) -> ParserResult<T>) -> ParserResult<T> {
        let ret = map(&self.tokens[self.cursor]);
        if ret.is_ok() {
            self.cursor += 1;
        }
        ret
    }

    fn eat_variant(&mut self, kind_variant: TokenKind) -> ParserResult<&Token> {
        let cur = &self.tokens[self.cursor];
        if std::mem::discriminant(&kind_variant) == std::mem::discriminant(&cur.kind) {
            self.cursor += 1;
            Ok(cur)
        } else {
            Err(ParserFault {
                loc: cur.source_location(),
            })
        }
    }

    fn one_of<T>(&mut self, parsers: &[fn(&mut Self) -> ParserResult<T>]) -> ParserResult<T> {
        let cursor = self.cursor;
        for parser in parsers {
            match parser(self) {
                Ok(good) => return Ok(good),
                Err(_err) => {
                    self.cursor = cursor;
                    continue;
                }
            }
        }

        Err(ParserFault {
            loc: self.current().source_location(),
        })
    }

    fn none_or_more<T>(&mut self, parser: fn(&mut Self) -> ParserResult<T>) -> Vec<T> {
        let mut results = vec![];
        let mut cursor = self.cursor;
        while let Ok(parsed) = parser(self) {
            results.push(parsed);
            cursor = self.cursor;
        }
        self.cursor = cursor;

        results
    }

    fn one_or_more<T>(&mut self, parser: fn(&mut Self) -> ParserResult<T>) -> ParserResult<Vec<T>> {
        match parser(self) {
            Ok(first) => {
                let mut results = vec![first];
                results.extend(self.none_or_more(parser));
                Ok(results)
            }
            Err(err) => Err(err),
        }
    }

    pub fn parse_stmt(&mut self) -> ParserResult<Statement> {
        self.one_of(&[
            |parser| parser.parse_while(),
            |parser| {
                let ret = parser.parse_name_decl()?;
                parser.eat_variant(TokenKind::Newline)?;
                Ok(ret)
            },
            |parser| {
                let ret = parser.parse_expression()?;
                parser.eat_variant(TokenKind::Newline)?;
                Ok(Statement {
                    loc: ret.source_location(),
                    span: ret.source_span(),
                    stmt: Stmt::Expression(ret.expr),
                })
            },
        ])
    }

    pub fn parse_while(&mut self) -> ParserResult<Statement> {
        let while_keyword = self.eat_variant(TokenKind::Keyword(Keyword::While))?;
        let span_begin = while_keyword.source_span().0;
        let loc = while_keyword.source_location();

        let pred = self.parse_expression()?;

        self.eat_variant(TokenKind::Punctuation(Punctuation::Colon))?;
        self.eat_variant(TokenKind::Newline)?;
        self.eat_variant(TokenKind::Indent)?;

        let body = self.parse_stmt_block()?;
        let span_end = body.span.1;

        self.eat_variant(TokenKind::Dendent)?;

        Ok(Statement {
            loc,
            span: (span_begin, span_end),
            stmt: Stmt::WhileStmt { pred, body },
        })
    }

    pub fn parse_name_decl(&mut self) -> ParserResult<Statement> {
        let let_keyword = self.eat_variant(TokenKind::Keyword(Keyword::Let))?;
        let span_begin = let_keyword.source_span().0;
        let loc = let_keyword.source_location();

        let name = self.eat(|token| match token {
            Token {
                kind: TokenKind::Identifier(name),
                ..
            } => Ok(name.to_owned()),
            _ => Err(ParserFault { loc: token.loc }),
        })?;

        self.eat_if(|token| matches!(token.kind, TokenKind::Operator(Operator::Equals)))?;

        let value = self.parse_expression()?;
        let span_end = value.source_span().1;

        Ok(Statement {
            loc: loc,
            span: (span_begin, span_end),
            stmt: Stmt::NameDeclaration { name, value },
        })
    }

    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        self.one_of(&[Self::parse_function_application, Self::parse_binop_expr])
    }

    pub fn parse_primary_expression(&mut self) -> ParserResult<Expression> {
        self.one_of(&[
            Parser::parse_integer,
            Parser::parse_name,
            Parser::parse_group,
        ])
    }

    pub fn parse_group(&mut self) -> ParserResult<Expression> {
        let _first = self.eat_variant(TokenKind::LeftParenthese)?;
        let span_begin = _first.source_span().0;
        let loc = _first.source_location();
        let expr = self.parse_expression()?;
        let _last = self.eat_variant(TokenKind::RightParenthese)?;
        let span_end = _last.source_span().1;

        Ok(Expression {
            span: (span_begin, span_end),
            expr: Expr::Grouping {
                expr: Box::new(expr),
            },
            loc,
        })
    }

    pub fn parse_function_application(&mut self) -> ParserResult<Expression> {
        let func = self.parse_name()?;
        let args = self.one_or_more(|p| {
            p.one_of(&[
                Parser::parse_integer,
                Parser::parse_name,
                Parser::parse_group,
            ])
        })?;

        Ok(Expression {
            loc: func.source_location(),
            span: (func.source_span().0, args.last().unwrap().span.1),
            expr: Expr::FunctionApplication(FunctionApplication {
                func: Box::new(func),
                args: args,
            }),
        })
    }

    pub fn parse_binop_expr(&mut self) -> ParserResult<Expression> {
        let mut output_stack: Vec<Expression> = vec![];
        let mut operator_stack: Vec<Operator> = vec![];

        let primary_starter = self.parse_primary_expression()?;
        output_stack.push(primary_starter);

        while let Ok(binop) = self.eat_if(Token::is_binop) {
            let op = match &binop.kind {
                TokenKind::Operator(op) => *op,
                _ => unreachable!(),
            };

            if !operator_stack.is_empty() {
                if precedence_of(&op) >= precedence_of(&operator_stack.last().unwrap()) {
                    let op = operator_stack.pop().unwrap();
                    let rhs = output_stack.pop().unwrap();
                    let lhs = output_stack.pop().unwrap();

                    let span = (lhs.source_span().0, rhs.source_span().1);
                    let loc = lhs.source_location();

                    output_stack.push(Expression {
                        loc,
                        span,
                        expr: Expr::Binop(BinopExpr {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }),
                    });
                }
            }

            operator_stack.push(op);
            output_stack.push(self.parse_primary_expression()?);
        }

        while !operator_stack.is_empty() {
            let op = operator_stack.pop().unwrap();
            let rhs = output_stack.pop().unwrap();
            let lhs = output_stack.pop().unwrap();

            let span = (lhs.source_span().0, rhs.source_span().1);
            let loc = lhs.source_location();

            output_stack.push(Expression {
                loc,
                span,
                expr: Expr::Binop(BinopExpr {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }),
            });
        }

        let expr = output_stack.pop().unwrap();

        Ok(expr)
    }

    pub fn parse_stmt_block(&mut self) -> ParserResult<StatementBlock> {
        let stmts = self.one_or_more(Parser::parse_stmt)?;

        if !stmts.is_empty() {
            // TODO: refactor the unwraps
            let loc = stmts.first().unwrap().source_location();
            let begin_span = stmts.first().unwrap().source_span().0;
            let end_span = stmts.last().unwrap().source_span().1;

            Ok(StatementBlock {
                loc,
                span: (begin_span, end_span),
                stmts,
            })
        } else {
            Err(ParserFault {
                loc: self.current().source_location(),
            })
        }
    }

    pub fn parse_ident(&mut self) -> ParserResult<String> {
        self.eat::<String>(|token| match token {
            Token {
                kind: TokenKind::Identifier(identifier),
                ..
            } => Ok(identifier.to_owned()),
            token => Err(ParserFault {
                loc: token.source_location(),
            }),
        })
    }

    pub fn parse_name(&mut self) -> ParserResult<Expression> {
        let (loc, span) = {
            let this = self.current();
            (this.source_location(), this.source_span())
        };

        self.parse_ident().map(|ident| Expression {
            loc,
            span,
            expr: Expr::Name(ident),
        })
    }

    pub fn parse_integer(&mut self) -> ParserResult<Expression> {
        self.eat_variant(TokenKind::Integer("".to_string()))
            .map(|token| {
                if let Token {
                    kind: TokenKind::Integer(int),
                    span,
                    loc,
                    ..
                } = token
                {
                    Expression {
                        span: span.to_owned(),
                        loc: loc.to_owned(),
                        expr: Expr::IntegerLiteral(int.to_owned()),
                    }
                } else {
                    unreachable!();
                }
            })
    }
}
