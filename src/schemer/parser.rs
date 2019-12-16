#![allow(unused)]

use super::tokens::{TokenInfo, Token, SpecialToken, TypeName};
use super::objects::*;

pub struct ParserState {
    current: usize,
    next: usize,
}

impl ParserState {
    pub fn new(cur: usize, nxt: usize) -> ParserState {
        ParserState {
            current: cur,
            next: nxt,
        }
    }
}

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    next: usize,
    eof_token: TokenInfo, 
}

mod helpers {
    use super::{ 
        Token, 
        Parser, 
        IntegerType, 
        FloatingType,
        StringType,
        BooleanType,
    };

    pub trait ValueReadCheck {
        fn token_checker(val: &Token) -> bool;
        fn expected() -> &'static str;
        fn read_value(&mut self, parser: &mut Parser);
    }

    impl ValueReadCheck for StringType {
        fn token_checker(val: &Token) -> bool {
            Token::is_string()(val)
        }

        fn expected() -> &'static str {
            "string"
        }

        fn read_value(&mut self, parser: &mut Parser) {
            match parser.current().token() {
                Token::String(val) => {
                    if !self.check_enum(val) {
                        parser.panic_current(&format!("Value '{}' is not valis for enum.", val));
                    }
                    self.add_value(val)
                },
                _ => ()
            }
        }
    }
    
    impl ValueReadCheck for IntegerType {
        fn token_checker(val: &Token) -> bool {
            Token::is_integer()(val)
        }
        fn expected() -> &'static str {
            "integer"
        }
        fn read_value(&mut self, parser: &mut Parser) {
            match parser.current().token() {
                Token::Integer(val) => {
                    if !self.check_enum(*val) {
                        parser.panic_current(&format!("Value {} is invalid for integer enum", *val));
                    } else if !self.check_minmax(*val) {
                        parser.panic_current(&format!("Value {} is invalid for integer interval", *val));
                    } else {
                        self.add_value(*val)
                    }
                },
                _ => ()
            }
        }
    }

    impl ValueReadCheck for FloatingType {
        fn token_checker(val: &Token) -> bool {
            Token::is_number()(val)
        }
        fn expected() -> &'static str {
            "floating or integer"
        }
        fn read_value(&mut self, parser: &mut Parser) {
            let val = match parser.current().token() {
                Token::Floating(val) => *val,
                Token::Integer(val) => *val as f64,
                _ => panic!("Should not be here")
            };
            if !self.check_enum(val) {
                parser.panic_current(&format!("Value {} is invalid for floating enum", val));
            } else if !self.check_minmax(val) {
                parser.panic_current(&format!("Value {} is invalid for floating interval", val));
            } else {
                self.add_value(val)
            }
        }
    }

    impl ValueReadCheck for BooleanType {
        fn token_checker(val: &Token) -> bool {
            Token::is_boolean()(val)
        }
        fn expected() -> &'static str {
            "true or false"
        }
        fn read_value(&mut self, parser: &mut Parser) {
            match parser.current().token() {
                Token::Boolean(val) => {
                    self.add_value(*val)
                },
                _ => ()
            }
        }
    }
}

impl Parser {
    pub fn new(toks: Vec<TokenInfo>) -> Parser {
        let len = toks.len();
        Parser {
            tokens: toks,
            current: 0,
            next: if len == 0 { 0 } else { 1 },
            eof_token: TokenInfo::new(Token::Eof, (len, len)),
        }
    }

    pub fn backup(&self) -> ParserState {
        ParserState::new(self.current, self.next)
    }

    pub fn restore(&mut self, bkup: &ParserState) {
        self.current = bkup.current;
        self.next = bkup.next;
    }

    pub fn advance(&mut self) -> bool {
        self.current = self.next;
        return if self.next == self.tokens.len() {
            false
        } else {
            self.next += 1;
            true
        }
    }

    pub fn eof(&self) -> bool {
        self.current == self.tokens.len()
    }

    pub fn next_eof(&self) -> bool {
        self.next == self.tokens.len()
    }

    pub fn current(&self) -> &TokenInfo {
        return if self.eof() { &self.eof_token } else { &self.tokens[self.current] };
    }

    pub fn next(&self) -> &TokenInfo {
        return if self.next_eof() { &self.eof_token } else { &self.tokens[self.next] };
    }

    fn panic_expect(&self, exp: &str) {
        panic!("unexpected '{}' at {}:{}. Expected '{}'", self.next().to_string(), 
            self.next().position().0, self.next().position().1, exp);
    }

    fn panic_current(&self, exp: &str) {
        panic!("current '{}' at {}:{}. {}", self.current().to_string(), 
            self.current().position().0, self.current().position().1, exp);
    }

    pub fn expect<F: Fn(&Token) -> bool>(&mut self, call: &F) -> bool {
        if call(self.next().token()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_begin<T: ObjectBase>(&mut self, mut result: T) -> T {
        if self.expect(&Token::is_special(SpecialToken::LBracket)) {
            if !self.expect(&Token::is_special(SpecialToken::RBracket)) {
                self.panic_expect("]");
            }
            result.set_array();
        }
        return result;
    }

    fn read_name(&mut self) -> String {
        let name = match &self.next().token() {
            Token::Ident(value) => { Some(String::from(value)) },
            Token::String(value) => { Some(String::from(value)) },
            _ => None,
        };
        match name {
            Some(value) => { self.advance(); value },
            None => { String::new() },
        }
    }

    /// 
    pub fn parse_string(&mut self) -> StringType {
        let mut result = self.parse_begin(StringType::new());
        self.read_value(&mut result);
        result
    }

    fn read_value<T: helpers::ValueReadCheck + ObjectBase>(&mut self, output: &mut T) {
        if self.expect(&Token::is_special(SpecialToken::Equal)) {
            if !output.is_array() {
                if !self.expect(&T::token_checker) {
                    self.panic_expect(T::expected());
                }
                output.read_value(self);
            } else {
                if !self.expect(&Token::is_special(SpecialToken::LBracket)) {
                    self.panic_expect("[");
                }
                self.try_read_array(output);
            }
        }
    }

    fn try_read_array<T: helpers::ValueReadCheck>(&mut self, output: &mut T) {
        while self.expect(&T::token_checker) {
            output.read_value(self);
            self.expect(&Token::is_special(SpecialToken::Comma));
        }
        if !self.expect(&Token::is_special(SpecialToken::RBracket)) {
            self.panic_expect(&(String::from("] or ") + T::expected()));
        }
    }

    fn parse_number<T: helpers::ValueReadCheck + ObjectBase>(&mut self, mut result: T) -> T {
        result = self.parse_begin(result);
        self.read_value(&mut result);
        return result;
    }

    pub fn parse_integer(&mut self) -> IntegerType {
        self.parse_number(IntegerType::new())
    } 
    
    pub fn parse_floating(&mut self) -> FloatingType {
        self.parse_number(FloatingType::new())
    }

    pub fn parse_boolean(&mut self) -> BooleanType {
        let mut result = self.parse_begin(BooleanType::new());
        self.read_value(&mut result);
        return result;
    }

    pub fn parse_object(&mut self) -> ObjectType {
        let mut result = self.parse_begin(ObjectType::new());
        if self.expect(&Token::is_special(SpecialToken::LBrace)) {
            while !self.expect(&Token::is_special(SpecialToken::RBrace)) {
                let element = self.parse_field();
                if result.has_field(element.name()) {
                    panic!("Field '{}' is already defined in onject.", element.name());
                }
                result.add_field(element);
            }
        }
        return result;
    }

    pub fn parse_field(&mut self) -> FieldType {
        let name = self.read_name();
        if name.len() > 0 && !self.expect(&Token::is_special(SpecialToken::Colon)) {
            self.panic_expect(":");
        }
        self.advance();
        let element = match &self.current().token() {
            Token::Type(name) => match name {
                TypeName::TypeString => Element::Str(self.parse_string()),
                TypeName::TypeInteger => Element::Integer(self.parse_integer()),
                TypeName::TypeFloating => Element::Floating(self.parse_floating()),
                TypeName::TypeBoolean => Element::Boolean(self.parse_boolean()),
                TypeName::TypeObject => Element::Object(self.parse_object()),
            },
            _ => { self.panic_current("typename"); Element::None }
        };
        FieldType::new(name, element)
    }
}
