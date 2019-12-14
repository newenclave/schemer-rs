#![allow(unused)]

use super::tokens::{TokenInfo, Token, SpecialToken};

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

    pub fn expect<F: Fn(&Token) -> bool>(&mut self, call: &F) -> bool {
        if call(self.next().token()) {
            self.advance();
            true
        } else {
            false
        }
    }
}
