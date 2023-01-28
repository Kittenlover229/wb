use crate::token::{SourceLocation, SourceSpan, Token};
use crate::tokenizer::{TokenStream};

#[derive(Debug, Clone, Copy)]
pub struct NonTokenizableSubstringError {
    pub loc: SourceLocation,
    pub span: SourceSpan,
}

pub type TokenizerResult = Result<Token, NonTokenizableSubstringError>;

pub struct ErrorAwareTokenStream<'a> {
    stream: TokenStream<'a>,
}

impl<'a> ErrorAwareTokenStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            stream: TokenStream::new(input),
        }
    }
}

impl<'a> Iterator for ErrorAwareTokenStream<'a> {
    type Item = TokenizerResult;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.stream.next();
        match next {
            Some(token) => Some(Ok(token)),
            None => {
                if let Some(input) = self.stream.input {
                    let ret = Some(TokenizerResult::Err(NonTokenizableSubstringError {
                        loc: self.stream.loc,
                        span: (self.stream.loc.index, self.stream.loc.index + input.len()),
                    }));
                    self.stream.input = None;
                    ret
                } else {
                    None
                }
            }
        }
    }
}
