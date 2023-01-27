use lex::{Keyword, Operator, Punctuation, SourceLocation, SourceObject, Token, TokenKind};

use crate::ast::{
    BinaryExpression, Expression, ExpressionStmt, IntegerLiteral, NameDeclStmt,
    NameDeclarationStatement, NameExpression, Statement, StatementBlock, WhileStatement, WhileStmt,
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
            |parser| Ok(WhileStmt(parser.parse_while()?)),
            |parser| {
                let ret = Ok(NameDeclStmt(parser.parse_var_decl()?))?;
                parser.eat_variant(TokenKind::Newline)?;
                Ok(ret)
            },
            |parser| {
                let ret = Ok(ExpressionStmt(parser.parse_expression()?))?;
                parser.eat_variant(TokenKind::Newline)?;
                Ok(ret)
            },
        ])
    }

    pub fn parse_while(&mut self) -> ParserResult<WhileStatement> {
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

        Ok(WhileStatement {
            pred,
            body,
            loc,
            span: (span_begin, span_end),
        })
    }

    /// WIP: this doesn't handle errors at all, but it hopes that it works
    pub fn parse_var_decl(&mut self) -> ParserResult<NameDeclarationStatement> {
        let let_keyword = self.eat_variant(TokenKind::Keyword(Keyword::Let))?;
        let source_span_begin = let_keyword.source_span().0;
        let loc = let_keyword.source_location();

        let varname = self.parse_name()?;

        self.eat_if(|token| matches!(token.kind, TokenKind::Operator(Operator::Equals)))?;

        let rhs = self.parse_expression()?;
        let source_span_end = rhs.source_span().1;

        Ok(NameDeclarationStatement {
            span: (source_span_begin, source_span_end),
            rhs: rhs,
            name: varname,
            loc,
        })
    }

    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        self.parse_binop_expr()
    }

    pub fn parse_primary_expression(&mut self) -> ParserResult<Expression> {
        self.one_of(&[
            |parser| parser.parse_integer().map(IntegerLiteral),
            |parser| parser.parse_name().map(NameExpression),
        ])
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

                    output_stack.push(Expression::BinaryExpression(BinaryExpression {
                        span,
                        loc,
                        operator: op,
                        rhs: Box::new(rhs),
                        lhs: Box::new(lhs),
                    }))
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

            output_stack.push(Expression::BinaryExpression(BinaryExpression {
                operator: op,
                span,
                loc,
                rhs: Box::new(rhs),
                lhs: Box::new(lhs),
            }))
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
                statements: stmts,
            })
        } else {
            Err(ParserFault {
                loc: self.current().source_location(),
            })
        }
    }

    pub fn parse_name(&mut self) -> ParserResult<NameExpression> {
        self.eat::<NameExpression>(|token| match token {
            Token {
                kind: TokenKind::Identifier(identifier),
                ..
            } => Ok(NameExpression {
                loc: token.source_location(),
                span: token.source_span(),
                identifier: identifier.to_owned(),
            }),
            token => Err(ParserFault {
                loc: token.source_location(),
            }),
        })
    }

    pub fn parse_integer(&mut self) -> ParserResult<IntegerLiteral> {
        self.eat_if(|t| matches!(t.kind, TokenKind::Integer(_)))
            .map(|token| {
                if let Token {
                    kind: TokenKind::Integer(int),
                    span,
                    loc,
                    ..
                } = token
                {
                    IntegerLiteral {
                        number: int.to_owned(),
                        span: span.to_owned(),
                        loc: loc.to_owned(),
                    }
                } else {
                    unreachable!();
                }
            })
    }
}
