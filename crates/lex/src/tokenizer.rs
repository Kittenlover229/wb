use crate::token::*;
use regex::Regex;

pub type TokenizerRule = dyn Fn(&str, SourceLocation) -> Option<(Token, &str, SourceLocation)>;

pub struct TokenStream<'a> {
    pub input: &'a str,

    tokenizer_rules: Vec<Box<TokenizerRule>>,
}

pub fn tokenize<'a>(input: &'a str) -> TokenStream<'a> {
    let tokenizer_rules: Vec<Box<TokenizerRule>> = vec![
        Box::new(|s, loc| {
            let re: Regex = Regex::new(r"^let").unwrap();
            let m = re.find(s)?;
            if m.start() != 0 {
                return None;
            }

            let (capture, rest) = s.split_at(m.end());
            Some((
                Token {
                    capture,
                    loc,
                    kind: TokenKind::Keyword,
                },
                rest,
                loc,
            ))
        }),
        Box::new(|s, loc| {
            let re: Regex = Regex::new(r"^\s+").unwrap();
            let m = re.find(s)?;
            if m.start() != 0 {
                return None;
            }

            let (capture, rest) = s.split_at(m.end());
            Some((
                Token {
                    capture,
                    loc,
                    kind: TokenKind::Whitespace,
                },
                rest,
                loc,
            ))
        }),
        Box::new(|s, loc| {
            let re: Regex = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
            let m = re.find(s)?;
            if m.start() != 0 {
                return None;
            }

            let (capture, rest) = s.split_at(m.end());
            Some((
                Token {
                    capture,
                    loc,
                    kind: TokenKind::Identifier,
                },
                rest,
                loc,
            ))
        }),
        Box::new(|s, loc| {
            let re: Regex = Regex::new(r"[=;]").unwrap();
            let m = re.find(s)?;
            if m.start() != 0 {
                return None;
            }

            let (capture, rest) = s.split_at(m.end());
            Some((
                Token {
                    capture,
                    loc,
                    kind: TokenKind::Punctuation,
                },
                rest,
                loc,
            ))
        }),
        Box::new(|s, loc| {
            let re: Regex = Regex::new(r"[0-9]+").unwrap();
            let m = re.find(s)?;
            if m.start() != 0 {
                return None;
            }

            let (capture, rest) = s.split_at(m.end());
            Some((
                Token {
                    capture,
                    loc,
                    kind: TokenKind::Integer,
                },
                rest,
                loc,
            ))
        }),
    ];

    TokenStream {
        input,
        tokenizer_rules,
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for rule in &self.tokenizer_rules {
            match rule(self.input, SourceLocation {}) {
                None => continue,
                Some((tok, rest, _loc)) => {
                    self.input = rest;
                    return Some(tok);
                }
            }
        }

        None
    }
}
