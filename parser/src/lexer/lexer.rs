use peekmore::{PeekMoreIterator, PeekMore};
use std::collections::HashMap;

use super::{token::{TokenType, Token}, constants::get_keywords};


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
            keywords: get_keywords()
        }
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];

        loop {
            match self.get_token() {
                Ok(token) => {
                    tokens.push(token.to_owned());
                    if token.is_type(TokenType::EOF) {
                        break;
                    }
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
                    '/' => {
                        if self.match_next('/') {
                            return self.handle_comment()
                        }
                        self.make_defualt_token(TokenType::Slash)
                    }

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
                    c if c.is_alphabetic() || matches!(c, '_') => self.handle_identifer(c),

                    _ => Err(LexerError { error: format!("Invalid character '{}'", c), line: self.line })
                }
            }
            None => self.make_defualt_token(TokenType::EOF),
        }
    }

    fn handle_comment(&mut self) -> Result<Token, LexerError> {
        while let Some(c) = self.source.peek() {
            if matches!(c, '\n') {
                break;
            }
            self.source.next();
        }
        self.get_token()
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

    fn handle_identifer(&mut self, start: char) -> Result<Token, LexerError> {
        let mut literal = String::from(start);

        while let Some(c) = self.source.peek() {
            if !(c.is_alphanumeric() || matches!(c, '_')) {
                break;
            }
            
            literal.push(*c);
            self.source.next();
        }

        match self.keywords.get(&literal) {
            Some(token_type) => self.make_defualt_token(*token_type),
            None => self.make_token(TokenType::Identifier, Some(literal))
        }
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


#[derive(Debug)]
pub struct LexerError {
    pub error: String,
    pub line: usize
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer Error: {} at line {}", self.error, self.line)
    }
}