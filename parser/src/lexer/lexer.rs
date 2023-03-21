use peekmore::{PeekMoreIterator, PeekMore};
use std::collections::HashMap;

use super::token::{TokenType, Token};


// TODO: Add later
// start: usize,
// end: usize
#[derive(Debug)]
pub struct Lexer<'a> {
    source: PeekMoreIterator<std::str::Chars<'a>>,
    // current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Lexer<'a> {
        Lexer {
            source: source.chars().peekmore(),
            // current: 0,
            line: 1,
            keywords: HashMap::from([
                (String::from("true"),      TokenType::True),
                (String::from("false"),     TokenType::False),
                (String::from("none"),      TokenType::None),
                (String::from("if"),        TokenType::If),
                (String::from("else"),      TokenType::Else),
                (String::from("loop"),      TokenType::Loop),
                (String::from("let"),       TokenType::Let),
                (String::from("const"),     TokenType::Const),
                (String::from("fn"),        TokenType::Fn),
                (String::from("class"),     TokenType::Class),
                (String::from("return"),    TokenType::Return),
                (String::from("self"),      TokenType::SelfTok),
                (String::from("and"),       TokenType::And),
                (String::from("or"),        TokenType::Or),
                (String::from("break"),     TokenType::Break),
                (String::from("continue"),  TokenType::Continue),
                (String::from("log"),       TokenType::LogFn),
                (String::from("type"),      TokenType::TypeFn),
                (String::from("num"),       TokenType::NumFn),
                (String::from("str"),       TokenType::StrFn),
                (String::from("bool"),      TokenType::BoolFn),
            ])
        }
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];

        loop {
            match self.get_token() {
                Ok(token) => {
                    if token.is_type(TokenType::EOF) {
                        tokens.push(token);
                        break;
                    }
                    tokens.push(token)
                }
                Err(err) => return Err(err)
            }
        }

        Ok(tokens)
    }

    fn get_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        match self.source.next() {
            Some(c) => {
                match c {
                    '(' => self.make_defualt_token(TokenType::LeftParen),
                    ')' => self.make_defualt_token(TokenType::RightParen),
                    '{' => self.make_defualt_token(TokenType::LeftCurly),
                    '}' => self.make_defualt_token(TokenType::RightCurly),
                    '[' => self.make_defualt_token(TokenType::LeftSqure),
                    ']' => self.make_defualt_token(TokenType::RightSqure),
                    '@' => self.make_defualt_token(TokenType::At),
                    ';' => self.make_defualt_token(TokenType::SemiColon),
                    ',' => self.make_defualt_token(TokenType::Comma),
                    '.' => {
                        if self.match_next('.') {
                            return self.make_defualt_token(TokenType::DotDot)
                        }
                        self.make_defualt_token(TokenType::Dot)
                    }

                    '+' => {
                        if self.match_next('=') {
                            return self.make_defualt_token(TokenType::PlusEqual)
                        }
                        self.make_defualt_token(TokenType::Plus)
                    }
                    '-' => {
                        if self.match_next('=') {
                            return self.make_defualt_token(TokenType::MinusEqual)
                        }
                        self.make_defualt_token(TokenType::Minus)
                    }
                    '*' => self.make_defualt_token(TokenType::Star),
                    '/' => self.make_defualt_token(TokenType::Slash),

                    '!' => {
                        if self.match_next('=') {
                            return self.make_defualt_token(TokenType::BangEqual)
                        }
                        self.make_defualt_token(TokenType::Bang)
                    }
                    '=' => {
                        if self.match_next('=') {
                            return self.make_defualt_token(TokenType::EqualEqual)
                        }
                        self.make_defualt_token(TokenType::Equal)
                    }
                    '>' => {
                        if self.match_next('=') {
                            return self.make_defualt_token(TokenType::GreaterEqual)
                        }
                        self.make_defualt_token(TokenType::Greater)
                    }
                    '<' => {
                        if self.match_next('=') {
                            return self.make_defualt_token(TokenType::LessEqual)
                        }
                        self.make_defualt_token(TokenType::Less)
                    }

                    '0'..='9' => self.handle_number(c),
                    '"' => self.handle_string(),

                    _ => Err(LexerError { error: format!("Invalid character '{}'", c), line: self.line })
                }
            }
            None => self.make_defualt_token(TokenType::EOF),
        }
    }

    fn handle_number(&mut self, start: char) -> Result<Token, LexerError> {
        let mut literal = String::from(start);

        while let Some(c) = self.source.peek() {
            if !c.is_digit(10) {
                break;
            }
            literal.push(*c);
            self.source.next();
        }

        if let Some(&'.') = self.source.peek() {
            self.source.advance_cursor();

            match self.source.peek() {
                Some(c) if c.is_digit(10) => {
                    literal.push('.');
                    self.source.next();

                    while let Some(c) = self.source.peek() {
                        if !c.is_digit(10) {
                            break;
                        }
                        literal.push(*c);
                        self.source.next();
                    }

                }
                _ => self.source.reset_cursor()
            }
        }

        self.make_token(TokenType::Number, Some(literal))
    }

    fn handle_string(&mut self) -> Result<Token, LexerError> {
        let mut literal = String::new();

        while let Some(c) = self.source.peek() {
            if matches!(c, '\n') {
                self.line += 1;
            }

            if matches!(c, '"') {
                self.source.next();
                return self.make_token(TokenType::String, Some(literal))
            }

            literal.push(*c);
            self.source.next();
        }

        Err(LexerError { error: String::from("Lexer Error: Unterminated string"), line: self.line })
    }

    fn make_token(&mut self, token_type: TokenType, literal: Option<String>) -> Result<Token, LexerError> {
        Ok(Token::new(token_type, literal, self.line))
    }

    fn make_defualt_token(&mut self, token_type: TokenType) -> Result<Token, LexerError> {
        self.make_token(token_type, None)
    }

    fn match_next(&mut self, next: char) -> bool {
        match self.source.peek() {
            Some(c) => {
                if *c == next {
                    self.source.next();
                    return true
                }
                false
            }
            None => false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.source.peek() {
            if c.is_whitespace() {
                if matches!(c, '\n') {
                    self.line += 1;
                }
                self.source.next();
            } else {
                break;
            }

        }
    }
}


pub struct LexerError {
    pub error: String,
    pub line: usize
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer Error: {} at line {}", self.error, self.line)
    }
}