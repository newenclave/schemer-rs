#![allow(unused)]

use super::tokens::{TokenInfo, Token, SpecialToken};
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

    pub fn expect<F: Fn(&Token) -> bool>(&mut self, call: &F) -> bool {
        if call(self.next().token()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn try_parse_str_array(&mut self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        while self.expect(&Token::is_string()) {
            match self.current().token() {
                Token::String(val) => res.push(val.to_string()),
                _ => ()
            }
            self.expect(&Token::is_special(SpecialToken::Comma));
        }
        if !self.expect(&Token::is_special(SpecialToken::RBracket)) {
            self.panic_expect("]");
        }
        return res;
    }

    /// 
    pub fn parse_string(&mut self) -> StringType {
        let mut result = StringType::new();
        if self.expect(&Token::is_special(SpecialToken::LBracket)) {
            if !self.expect(&Token::is_special(SpecialToken::RBracket)) {
                self.panic_expect("]");
            }
            result.set_array(Vec::new());
        }
        if self.expect(&Token::is_ident()) {
            match self.current().token() {
                Token::Ident(value) => result.set_name(value),
                _ => (),
            }
        }

        if self.expect(&Token::is_special(SpecialToken::Equal)) {
            if !result.is_array() {
                if !self.expect(&Token::is_string()) {
                    self.panic_expect("string");
                }
                match self.current().token() {
                    Token::String(value) => result.add_value(value),
                    _ => (),
                }
            } else {
                if !self.expect(&Token::is_special(SpecialToken::LBracket)) {
                    self.panic_expect("]");
                }
                result.set_array(self.try_parse_str_array());
            }
        }
        result
    }
}
