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
                    '(' => self.make_token(TokenType::LeftParen, None),
                    ')' => self.make_token(TokenType::RightParen, None),
                    '{' => self.make_token(TokenType::LeftCurly, None),
                    '}' => self.make_token(TokenType::RightCurly, None),
                    '[' => self.make_token(TokenType::LeftSqure, None),
                    ']' => self.make_token(TokenType::RightSqure, None),
                    '@' => self.make_token(TokenType::At, None),
                    ';' => self.make_token(TokenType::SemiColon, None),
                    ',' => self.make_token(TokenType::Comma, None),
                    '.' => self.make_token(TokenType::Dot, None),

                    '+' => self.make_token(TokenType::Plus, None),
                    '-' => self.make_token(TokenType::Minus, None),
                    '*' => self.make_token(TokenType::Star, None),
                    '/' => self.make_token(TokenType::Slash, None),

                    '!' => self.make_token(TokenType::Bang, None),
                    '=' => self.make_token(TokenType::Equal, None),
                    '>' => self.make_token(TokenType::Greater, None),
                    '<' => self.make_token(TokenType::Less, None),

                    _ => Err(LexerError { error: format!("Invalid character '{}'", c), line: self.line })
                }
            }
            None => return Ok(Token::new(TokenType::EOF, None, self.line)),
        }
    }

    fn make_token(&mut self, token_type: TokenType, literal: Option<String>) -> Result<Token, LexerError> {
        Ok(Token::new(token_type, literal, self.line))
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