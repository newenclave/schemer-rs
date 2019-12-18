#![allow(unused)]

#[derive(Clone, PartialEq)]
pub enum SpecialToken {
    LParen, // (
    RParen, // )

    LBrace, // {
    RBrace, // }

    LBracket, // [
    RBracket, // ]

    Equal, // =
    Comma, // ,
    Colon, // :
    Semicolon, // ;

    Interval, // ..
}

#[derive(Clone, PartialEq)]
pub enum TypeName {
    TypeString, // string
    TypeInteger, // interger
    TypeFloating, // floating
    TypeBoolean, // boolean
    TypeObject, // object
}

#[derive(Clone, PartialEq)]
pub enum Token {
    None, 
    Ident(String),
    Integer(i64),
    Floating(f64),
    String(String),
    Boolean(bool),
    Type(TypeName),
    Special(SpecialToken),
    Eof,
}

impl Token {
    pub fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false
        }
    }
    pub fn is_floating(&self) -> bool {
        match self {
            Token::Floating(_) => true,
            _ => false
        }
    }
    pub fn is_integer(&self) -> bool {
        match self {
            Token::Integer(_) => true,
            _ => false
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            Token::Integer(_) => true,
            Token::Floating(_) => true,
            _ => false
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Token::String(_) => true,
            _ => false
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Token::Boolean(_) => true,
            _ => false
        }
    }
    pub fn is_eof(&self) -> bool {
        match self {
            Token::Eof => true,
            _ => false
        }
    }
    pub fn is_special(val: SpecialToken) -> impl Fn(&Token) -> bool {
        move |tok: &Token| match tok {
            Token::Special(s) => {
                *s == val
            },
            _ => false
        }
    }
    pub fn is_type(&self) -> bool {
        match self {
            Token::Type(t) => true,
            _ => false
        }
    }
}

#[derive(Clone)]
pub struct TokenInfo {
    token: Token,
    position: (usize, usize),
    literal: String,
}

impl TokenInfo {
    pub fn new(value: Token, pos: (usize, usize)) -> TokenInfo {
        TokenInfo {
            token: value,
            position: pos,
            literal: "".to_string(),
        }
    } 

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn to_string(&self) -> String {
        match &self.token {
            Token::None => format!("None"),
            Token::Ident(ident) => format!("{}", ident),
            Token::Integer(i) => format!("{}", i),
            Token::Floating(f) => format!("{}", f),
            Token::String(s) => format!("\"{}\"", s),
            Token::Boolean(b) => (if *b { "true" } else { "false" }).to_string(),
            Token::Type(t) => match t {
                TypeName::TypeString => "string".to_string(),
                TypeName::TypeInteger => "integer".to_string(),
                TypeName::TypeFloating => "floating".to_string(),
                TypeName::TypeBoolean => "boolean".to_string(),
                TypeName::TypeObject => "object".to_string(),
            }
            Token::Special(spec) => match spec { 
                SpecialToken::LParen => "(".to_string(),
                SpecialToken::RParen => ")".to_string(),
                SpecialToken::LBrace => "{".to_string(),
                SpecialToken::RBrace => "}".to_string(),
                SpecialToken::LBracket => "[".to_string(),
                SpecialToken::RBracket => "]".to_string(),
                SpecialToken::Equal => "=".to_string(),
                SpecialToken::Comma => ",".to_string(),
                SpecialToken::Colon => ":".to_string(),
                SpecialToken::Semicolon => ";".to_string(),
                SpecialToken::Interval => "..".to_string(),
            },
            Token::Eof => "eof".to_string(),
        }
    }
}

