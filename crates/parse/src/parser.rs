use lex::{Keyword, Operator, Punctuation, SourceLocation, SourceObject, Token, TokenKind};

use crate::ast::{
    BinaryExpression, Expression, IntegerLiteral, Statement, VarDeclStatement, VarDeclStmt,
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

    fn peek_and_eat(&mut self) -> &Token {
        let ret = &self.tokens[self.cursor];
        self.cursor += 1;
        return ret;
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
        for parser in parsers {
            match parser(self) {
                Ok(good) => return Ok(good),
                Err(_err) => continue,
            }
        }

        Err(ParserFault {
            loc: self.current().source_location(),
        })
    }

    pub fn parse_stmt(&mut self) -> ParserResult<Statement> {
        self.one_of(&[|parser| {
            let vardecl = parser.parse_var_decl()?;
            parser.eat_variant(TokenKind::Newline)?;
            Ok(VarDeclStmt(vardecl))
        }])
    }

    /// WIP: this doesn't handle errors at all, but it hopes that it works
    pub fn parse_var_decl(&mut self) -> ParserResult<VarDeclStatement> {
        let token = self.eat_if(|t| t.kind == TokenKind::Keyword(Keyword::Let))?;
        let source_span_begin = token.source_span().0;
        let loc = token.source_location();

        let varname = (self.eat::<String>(|token| match token {
            Token {
                kind: TokenKind::Identifier(string),
                ..
            } => Ok(string.to_owned()),
            token => Err(ParserFault {
                loc: token.source_location(),
            }),
        }))?;

        self.eat_if(|token| matches!(token.kind, TokenKind::Punctuation(Punctuation::Equals)))?;

        let rhs = self.parse_expression()?;
        let source_span_end = rhs.source_span().1;

        Ok(VarDeclStatement {
            span: (source_span_begin, source_span_end),
            rhs: rhs,
            varname,
            loc,
        })
    }

    pub fn parse_expression(&mut self) -> ParserResult<Expression> {
        self.one_of(&[Parser::parse_binop_expr, Parser::parse_primary_expression])
    }

    pub fn parse_primary_expression(&mut self) -> ParserResult<Expression> {
        self.parse_integer().map(IntegerLiteral)
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
