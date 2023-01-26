use crate::token::{SourceLocation, Token, SourceSpan};
use regex::Regex;

pub trait TokenizerRule {
    fn try_tokenize<'a>(
        &self,
        input: &'a str,
        loc: SourceLocation,
    ) -> Option<(Token, &'a str, SourceLocation)>;
}

pub struct RegexTokenizerRule {
    regex: Regex,
    token_from: Box<dyn Fn(&str,SourceSpan, SourceLocation) -> Token>,
}

impl RegexTokenizerRule {
    pub fn new(regex: Regex, token_from: Box<dyn Fn(&str, SourceSpan, SourceLocation) -> Token>) -> Self {
        RegexTokenizerRule { regex, token_from }
    }

    pub fn new_box(regex: Regex, token_from: Box<dyn Fn(&str, SourceSpan, SourceLocation) -> Token>) -> Box<Self> {
        Box::new(Self::new(regex, token_from))
    }
}

impl TokenizerRule for RegexTokenizerRule {
    fn try_tokenize<'a>(
        &self,
        input: &'a str,
        mut loc: SourceLocation,
    ) -> Option<(Token, &'a str, SourceLocation)> {
        let m = self.regex.find(input)?;
        if m.start() != 0 {
            return None;
        }

        let (capture, rest) = input.split_at(m.end());
        let tok = self.token_from.as_ref()(capture, (loc.index, loc.index + m.end()), loc);
        loc.advance(input, capture.len());
        return Some((tok, rest, loc));
    }
}
