use std::collections::HashMap;

use crate::TokenType;


pub(crate) fn get_keywords() -> HashMap<String, TokenType> {
  HashMap::from([
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
