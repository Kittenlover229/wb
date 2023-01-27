use crate::token::{Token, TokenKind};

pub fn indented_tokens<'a>(mut iterator: impl Iterator<Item = Token>) -> Vec<Token> {
    let mut prev = match iterator.next() {
        Some(x) => x,
        None => return vec![],
    };

    assert!(
        std::mem::discriminant(&prev.kind) != std::mem::discriminant(&TokenKind::Whitespace(0xF00))
    );

    let mut indentation = 0;

    // XXX: handle the case when file starts with whitespaces
    let mut out = vec![];
    loop {
        let next = iterator.next();
        match (prev, next) {
            (
                Token {
                    kind: TokenKind::Newline,
                    span: newline_span,
                    loc: newline_loc,
                },
                Some(Token {
                    kind: TokenKind::Whitespace(spaces),
                    span,
                    loc,
                    ..
                }),
            ) => {
                out.push(Token {
                    loc: newline_loc,
                    span: newline_span,
                    kind: TokenKind::Newline,
                });

                let indent_spaces = spaces / 4;
                if indent_spaces == indentation + 1 {
                    indentation = indent_spaces;
                    out.push(Token {
                        kind: TokenKind::Indent,
                        span,
                        loc,
                    });
                } else if indent_spaces < indentation {
                    for _ in 0..(indentation - indent_spaces) {
                        out.push(Token {
                            kind: TokenKind::Dendent,
                            span,
                            loc,
                        });
                    }
                    indentation = indent_spaces;
                } else if indent_spaces > indentation {
                    todo!()
                }

                if let Some(next) = iterator.next() {
                    prev = next;
                } else {
                    break;
                }
            }

            (
                Token {
                    kind: TokenKind::Newline,
                    span,
                    loc,
                },
                Some(next),
            ) => {
                out.push(Token {
                    kind: TokenKind::Newline,
                    span,
                    loc,
                });
                for _ in 0..indentation {
                    out.push(Token {
                        kind: TokenKind::Dendent,
                        span,
                        loc,
                    });
                }
                indentation = 0;
                prev = next;
            }

            (
                Token {
                    kind: TokenKind::Newline,
                    span,
                    loc,
                },
                None,
            ) => {
                out.push(Token {
                    kind: TokenKind::Newline,
                    span,
                    loc,
                });
                for _ in 0..indentation {
                    out.push(Token {
                        kind: TokenKind::Dendent,
                        span,
                        loc,
                    });
                }
                break;
            }

            (last_tok, None) => {
                out.push(last_tok);
                break;
            }

            (tok, Some(next)) => {
                let temp = tok;
                prev = next;
                out.push(temp);
            }
            _ => unreachable!(),
        }
    }

    out
}
