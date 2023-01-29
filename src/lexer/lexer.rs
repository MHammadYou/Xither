use std::collections::HashMap;

use super::token::TokenType;


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
            keywords: HashMap::new()
        }
    }
}