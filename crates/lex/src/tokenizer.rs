use regex::Regex;

use crate::{
    rules::{RegexTokenizerRule, TokenizerRule},
    token::*,
};

pub struct TokenStream<'a> {
    pub input: Option<&'a str>,
    pub loc: SourceLocation,

    tokenizer_rules: Vec<Box<dyn TokenizerRule>>,
}

pub fn try_tokenize<'a>(input: &'a str) -> TokenStream<'a> {
    fn string_to_op(string: &str) -> Operator {
        match string {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "%" => Operator::Mod,
            ">" => Operator::Greater,
            "<" => Operator::Less,
            "=" => Operator::Equals,
            _ => unreachable!(),
        }
    }

    let tokenizer_rules: Vec<Box<dyn TokenizerRule>> = vec![
        RegexTokenizerRule::new_box(
            Regex::new(r"^(let|while)").unwrap(),
            Box::new(|captured: &str, span, loc| {
                use Keyword::*;
                Token {
                    loc,
                    span,
                    kind: TokenKind::Keyword(match captured {
                        "let" => Let,
                        "while" => While,
                        _ => unreachable!(),
                    }),
                }
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[0-9_]+").unwrap(),
            Box::new(|captured, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Integer(captured.to_string()),
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[ \t]+").unwrap(),
            Box::new(|captured, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Whitespace(captured.len() as u32),
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[\n\r]").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Newline,
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^\(").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::LeftParenthese,
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^\)").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::RightParenthese,
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
            Box::new(|captured, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Identifier(captured.to_string()),
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[-\+\\*%><]=").unwrap(),
            Box::new(|captured, span, loc| Token {
                loc,
                span,
                kind: TokenKind::CompoundOperator(string_to_op(&captured[..captured.len() - 1])),
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[-\+\\*=%><]").unwrap(),
            Box::new(|captured, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Operator(string_to_op(captured)),
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[;:]").unwrap(),
            Box::new(|capture, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Punctuation(match capture {
                    ":" => Punctuation::Colon,
                    ";" => Punctuation::Semicolon,
                    _ => unreachable!(),
                }),
            }),
        ),
    ];

    TokenStream {
        input: Some(input),
        loc: Default::default(),
        tokenizer_rules,
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input {
            Some(input) => {
                for rule in &self.tokenizer_rules {
                    match rule.try_tokenize(input, self.loc) {
                        Some((tok, rest, loc)) => {
                            let ret = Some(tok);
                            self.input = Some(rest);
                            self.loc = loc;
                            return ret;
                        }
                        None => continue,
                    }
                }

                if input.len() == 0 {
                    self.input = None;
                    Some(Token {
                        loc: self.loc,
                        span: (self.loc.index, self.loc.index),
                        kind: TokenKind::End,
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
