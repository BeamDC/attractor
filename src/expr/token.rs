#[derive(PartialEq, Debug)]
pub enum TokenKind {
    // Single char
    Comma,
    Colon,
    Semicolon,
    Pound,
    Dollar,
    At,
    Underscore,
    Lparen,
    Rparen,
    Lsquare,
    Rsquare,
    Lcurly,
    Rcurly,
    Dot,
    Add,
    Sub,
    Mul,
    Div,

    Bang,
    Eq,
    To,
    Gets,
    Implies,
    Iff, // todo: not yet handled
    Neq,
    Gt,
    Lt,
    Ge,
    Le,

    Number,
    Ident,

    // Misc
    Error, Eof, Unknown,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub raw: String,
    pub start: usize,
    pub end: usize,
}

