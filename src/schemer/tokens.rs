#![allow(unused)]

#[derive(Clone, PartialEq)]
pub enum SpecialToken {
    LParen, // (
    RParen, // )

    LBrace, // {
    RBrace, // }

    LBracket, // [
    RBracket, // ]

    TypeString, // string
    TypeInteger, // interger
    TypeFloating, // floating
    TypeBoolean, // boolean
    TypeObject, // object    
}

#[derive(Clone)]
pub enum Token {
    None, 
    Ident(String),
    Integer(i64),
    Floating(f64),
    String(String),
    Special(SpecialToken),
    Eof,
}

impl Token {
    pub fn is_ident() -> impl Fn(&Token) -> bool {
        |tok: &Token| match tok {
            Token::Ident(_) => true,
            _ => false
        }
    }
    pub fn is_floating() -> impl Fn(&Token) -> bool {
        |tok: &Token| match tok {
            Token::Floating(_) => true,
            _ => false
        }
    }
    pub fn is_integer() -> impl Fn(&Token) -> bool {
        |tok: &Token| match tok {
            Token::Integer(_) => true,
            _ => false
        }
    }
    pub fn is_number() -> impl Fn(&Token) -> bool {
        |tok: &Token| match tok {
            Token::Integer(_) => true,
            Token::Floating(_) => true,
            _ => false
        }
    }
    pub fn is_string() -> impl Fn(&Token) -> bool {
        |tok: &Token| match tok {
            Token::String(_) => true,
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
}

#[derive(Clone)]
pub struct TokenInfo {
    token: Token,
    position: (usize, usize)
}

impl TokenInfo {
    pub fn new(value: Token, pos: (usize, usize)) -> TokenInfo {
        TokenInfo {
            token: value,
            position: pos
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
            Token::Special(spec) => match spec { 
                SpecialToken::LParen => "(".to_string(),
                SpecialToken::RParen => ")".to_string(),
                SpecialToken::LBrace => "{".to_string(),
                SpecialToken::RBrace => "}".to_string(),
                SpecialToken::LBracket => "[".to_string(),
                SpecialToken::RBracket => "]".to_string(),
                SpecialToken::TypeString => "string".to_string(),
                SpecialToken::TypeInteger => "integer".to_string(),
                SpecialToken::TypeFloating => "floating".to_string(),
                SpecialToken::TypeBoolean => "boolean".to_string(),
                SpecialToken::TypeObject => "object".to_string(),
            },
            Token::Eof => "eof".to_string(),
        }
    }
}

