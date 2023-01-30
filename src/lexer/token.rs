

#[derive(Debug)]
pub enum TokenType {
    // Grammar
    LeftParen, RightParen,
    LeftCurly, RightCurly,
    LeftSqure, RightSqure,
    SemiColon, Comma, Dot, DotDot,

    // Operators
    Plus, Minus,
    Star, Slash,
    Less, LessEqual,
    Greater, GreaterEqual,
    Equal, EqualEqual,
    Bang, BangEqual,
    PlusEqual, MinusEqual,

    // Literals
    Identifier, String, Number,
    None, True, False,

    // Reserved keywords
    If, Else, Loop, Let, Const,
    Fn, Class, Return, At, SelfTok,
    And, Or, Break, Continue,

    // Built-ins
    LogFn, TypeFn, NumberFn, 
    StringFn, BooleanFn,

    EOF
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            line
        }
    }
}