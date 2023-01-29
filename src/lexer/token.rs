

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