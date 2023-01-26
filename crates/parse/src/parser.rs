use lex::{Keyword, SourceObject, Token, TokenKind};

use crate::ast::{IntegerLiteral, VarDeclStatement};

pub struct Parser {
    cursor: usize,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
pub struct ParserFault {}

pub type ParserResult<T> = Result<T, ParserFault>;

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

    fn eat_if(
        &mut self,
        pred: impl FnOnce(&Token) -> bool,
        make_err: impl FnOnce(&Token) -> ParserFault,
    ) -> ParserResult<&Token> {
        let cur = &self.tokens[self.cursor];
        if pred(cur) {
            self.cursor += 1;
            Ok(cur)
        } else {
            Err(make_err(cur))
        }
    }

    fn eat<T>(&mut self, map: impl FnOnce(&Token) -> ParserResult<T>) -> ParserResult<T> {
        let ret = map(&self.tokens[self.cursor]);
        if ret.is_ok() {
            self.cursor += 1;
        }
        ret
    }

    fn eat_variant(
        &mut self,
        kind_variant: TokenKind,
        make_err: impl FnOnce(&Token) -> ParserFault,
    ) -> ParserResult<&Token> {
        let cur = &self.tokens[self.cursor];
        if std::mem::discriminant(&kind_variant) == std::mem::discriminant(&cur.kind) {
            self.cursor += 1;
            Ok(cur)
        } else {
            Err(make_err(cur))
        }
    }

    /// WIP: this doesn't handle errors at all, but it hopes that it works
    pub fn parse_var_decl(&mut self) -> ParserResult<VarDeclStatement> {
        let token = self.eat_if(|t| t.kind == TokenKind::Keyword(Keyword::Let), |_| todo!())?;
        let source_span_begin = token.source_span().0;
        let loc = token.source_location();

        let varname = (self.eat::<String>(|token| {
            println!("{:?}", token);
            match token {
            Token {
                kind: TokenKind::Identifier(string),
                ..
            } => Ok(string.to_owned()),
            _ => Err(ParserFault {}),
        }}))?;

        self.eat_variant(TokenKind::Punctuation, |_| todo!())?;

        let rhs = self.parse_integer()?;
        let source_span_end = rhs.source_span().1;

        Ok(VarDeclStatement {
            span: (source_span_begin, source_span_end),
            rhs: IntegerLiteral(rhs),
            varname,
            loc,
        })
    }

    pub fn parse_integer(&mut self) -> ParserResult<IntegerLiteral> {
        self.eat_if(|t| matches!(t.kind, TokenKind::Integer(_)), |_| todo!())
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
