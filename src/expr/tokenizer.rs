use std::iter::Peekable;
use std::str::Chars;
use crate::expr::token::{Token, TokenKind};

/// helper macro to advance the tokenizer
/// constantly until the pattern is broken
macro_rules! advance_while {
    ($tokenizer:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        while let Some(c) = $tokenizer.peek() {
            match c {
                $pattern $(if $guard)? => {
                    $tokenizer.advance();
                },
                _ => break,
            }
        }
    };
}

pub struct Tokenizer<'a> {
    input: String,
    start: usize,
    pos: usize,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            input: src.to_string(),
            pos: 0,
            start: 0,
            chars: src.chars().peekable(),
        }
    }

    #[inline(always)]
    pub fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.chars.next()
    }
    
    #[inline(always)]
    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
                self.start = self.pos;
                continue;
            }
            break;
        }

        if let Some(c) = self.advance() {
            let kind = match c {
                ',' => TokenKind::Comma,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semicolon,
                '#' => TokenKind::Pound,
                '$' => TokenKind::Dollar,
                '@' => TokenKind::At,
                '_' => TokenKind::Underscore,
                '(' => TokenKind::Lparen,
                ')' => TokenKind::Rparen,
                '[' => TokenKind::Lsquare,
                ']' => TokenKind::Rsquare,
                '{' => TokenKind::Lcurly,
                '}' => TokenKind::Rcurly,
                '.' => TokenKind::Dot,

                '=' => match self.peek() {
                    Some('>') => {
                        self.advance();
                        TokenKind::Implies
                    },
                    _ => TokenKind::Eq
                },

                '!' => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            TokenKind::Neq
                        },
                        _ => TokenKind::Bang
                    }
                },
                '<' => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            TokenKind::Le
                        },
                        Some('-') => {
                            self.advance();
                            TokenKind::Gets
                        },
                        _ => TokenKind::Lt
                    }
                },
                '>' => {
                    match self.peek() {
                        Some('=') => {
                            self.advance();
                            TokenKind::Ge
                        },
                        _ => TokenKind::Gt
                    }
                },

                '+' => TokenKind::Add,
                '-' => TokenKind::Sub,
                '*' => TokenKind::Mul,
                '/' => TokenKind::Div,

                '0'..'9' => {
                    advance_while!(self, '0'..='9');
                    TokenKind::Number
                },

                'A'..='Z' | 'a'..='z' => {
                    advance_while!(self, 'A'..='Z' | 'a'..='z');
                    TokenKind::Ident
                },

                _ => TokenKind::Unknown
            };

            Token {
                kind,
                raw: self.input[self.start..self.pos].to_string(),
                start: self.start,
                end: self.pos,
            }
        } else {
            Token {
                kind: TokenKind::Eof,
                raw: "".to_string(),
                start: self.start,
                end: self.pos,
            }
        }

    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            // if last token is end, break
            let t = self.next_token();
            match t.kind {
                TokenKind::Eof => {
                    break;
                }
                _ => tokens.push(t),
            }
            self.start = self.pos;
        }
        tokens
    }
}