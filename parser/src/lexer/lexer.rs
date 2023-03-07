use std::collections::HashMap;

use super::token::{TokenType, Token};


#[derive(Debug)]
pub struct Lexer<'a> {
    pub source: PeekMoreIterator<std::str::Chars<'a>>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
    pub keywords: HashMap<String, TokenType>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Lexer<'a> {
        Lexer {
            source: source.chars().peekmore(),
            start: 0,
            current: 0,
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

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        tokens
    }
}