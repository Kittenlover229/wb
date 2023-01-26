use regex::Regex;

use crate::{
    rules::{RegexTokenizerRule, TokenizerRule},
    token::*,
};

pub struct TokenStream<'a> {
    pub input: &'a str,
    pub loc: SourceLocation,

    tokenizer_rules: Vec<Box<dyn TokenizerRule>>,
}

pub fn try_tokenize<'a>(input: &'a str) -> TokenStream<'a> {
    let tokenizer_rules: Vec<Box<dyn TokenizerRule>> = vec![
        RegexTokenizerRule::new_box(
            Regex::new(r"^let").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Keyword,
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^\s+").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Whitespace,
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Identifier,
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[=;]").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Punctuation,
            }),
        ),
        RegexTokenizerRule::new_box(
            Regex::new(r"^[0-9]+").unwrap(),
            Box::new(|_, span, loc| Token {
                loc,
                span,
                kind: TokenKind::Integer,
            }),
        ),
    ];

    TokenStream {
        input,
        loc: Default::default(),
        tokenizer_rules,
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        for rule in &self.tokenizer_rules {
            match rule.try_tokenize(self.input, self.loc) {
                Some((tok, rest, loc)) => {
                    let ret = Some(tok);
                    self.input = rest;
                    self.loc = loc;
                    return ret;
                }
                None => continue,
            }
        }

        None
    }
}
