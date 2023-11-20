

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Grammar
    LeftParen, RightParen,
    LeftCurly, RightCurly,
    LeftSqure, RightSqure, At,
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
    Fn, Class, Return, SelfTok,
    And, Or, Break, Continue,

    // Built-ins
    LogFn, TypeFn, NumFn, 
    StrFn, BoolFn,

    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
    pub line: usize
}

impl Token {
    pub(crate) fn new(token_type: TokenType, literal: Option<String>, line: usize) -> Token {
        Token { token_type, literal, line }
    }

    pub(crate) fn is_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}