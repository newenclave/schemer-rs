
use super::trie::Trie as Trie;
use super::scanner::Scanner as Scanner;
use super::tokens::{TokenInfo, Token, SpecialToken, TypeName};

struct TokenPack<T> {
    value: T,
    possible_ident: bool 
}

impl<T> TokenPack<T> {
    fn new(val: T, ident: bool) -> TokenPack<T> {
        TokenPack {
            value: val,
            possible_ident: ident,
        }
    }
}

pub struct Lexer {
    trie: Trie<TokenPack<Token>>
} 

mod helpers {
    use super::Scanner;
    pub enum Number {
        Integer(i64),
        Floating(f64),
    }
    pub fn scan_number(scan: &mut Scanner) -> Number {

        let mut d: i64 = 0;
        let mut a: f64 = 0.0;
        let mut e: i64 = 0;
        
        while !scan.eol() && scan.top().is_digit(10) {
            let value = scan.top().to_digit(10).unwrap();
            
            d *= 10;
            d += value as i64;
    
            a *= 10.0;
            a += value as f64;
            scan.advance();
        }
    
        let mut found: bool = false;
        let scan_bu = scan.backup(); 
    
        if scan.top() == '.' {
            scan.advance();
            while !scan.eol() && scan.top().is_digit(10) {
                found = true;
                a *= 10.0;
                a += scan.top().to_digit(10).unwrap() as f64;
                e -= 1;
                scan.advance();
            }
        }
    
        if scan.top() == 'e' || scan.top() == 'E' {
            let mut sign: i64 = 1;
            let mut i: i64 = 0;
            scan.advance();
            match scan.top() {
                '+' => { scan.advance(); },
                '-' => { scan.advance(); sign = -1 },
                 _  => { }
            }
            while !scan.eol() && scan.top().is_digit(10) {
                found = true;
                i *= 10;
                i += scan.top().to_digit(10).unwrap() as i64;
                scan.advance();
            }
            e += i * sign;
        }
    
        while e > 0 {
            a *= 10.0;
            e -= 1;
        }
    
        while e < 0 {
            a *= 0.1;
            e += 1;
        }
        return if found {
            Number::Floating(a)
        } else {
            scan.restore(&scan_bu);
            Number::Integer(d)
        }
    }

    pub fn str_head_tail(data: &str) -> (char, &str) {
        match data.chars().next() {
            Some(c) => (c, &data[c.len_utf8()..]),
            None => ('\0', data),
        }
    }

    pub fn is_ident(c: char) -> bool {
        return c.is_ascii_alphabetic() || c == '_';
    }

    pub fn scan_ident(scanner: &mut Scanner) -> String {
        let base = scanner.backup();
        let shift = scanner.advance_while(|c| { is_ident(c) || c.is_digit(10) });
        return String::from(&base.get()[0..shift])
    }

    pub fn scan_string(scan: &mut Scanner, ending: &str) -> String {

        let mut result = String::new();
        let ec = ending.chars().next().unwrap_or('\0');

        while !scan.eol() {
            if scan.get().starts_with(ending) {
                scan.jump(ending.len());
                break;
            }
            match scan.top() {
                '\\' => {
                    scan.advance();
                    match scan.top() {
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        '\0' => result.push('\0'),
                        val => if val == ec {
                                result.push(ec);
                            } else {
                                result.push('\\'); 
                                result.push(val);    
                            },
                    }
                },
                val => result.push(val),
            }
            scan.advance();
        }

        return result;
    }

    fn is_ident_string_rest(data: &str) -> bool {
        data.chars().find(|&x| !(x.is_digit(10) || is_ident(x))).is_none()
    }

    pub fn is_ident_string(data: &str) -> bool {
        let (head, tail) = str_head_tail(data);
        return is_ident(head) && is_ident_string_rest(tail);
    }

    pub fn skip_spaces(scanner: &mut Scanner) {
        scanner.advance_while(|c| { c.is_whitespace() });
    }
}

use helpers::*;

impl Lexer {
    pub fn new() -> Lexer {
        let mut lex = Lexer {
            trie: Trie::new()
        };

        lex.add_special("(", SpecialToken::LParen);
        lex.add_special(")", SpecialToken::RParen);
        lex.add_special("{", SpecialToken::LBrace);
        lex.add_special("}", SpecialToken::RBrace);
        lex.add_special("[", SpecialToken::LBracket);
        lex.add_special("]", SpecialToken::RBracket);

        lex.add_special("=", SpecialToken::Equal);
        lex.add_special(",", SpecialToken::Comma);
        lex.add_special("..", SpecialToken::Interval);
        lex.add_special(":", SpecialToken::Colon);
        lex.add_special(";", SpecialToken::Semicolon);
        lex.add_special("#", SpecialToken::Hash);

        lex.add_special("enum", SpecialToken::Enum);
        lex.add_special("null", SpecialToken::Null);

        lex.add_type("string", TypeName::TypeString);
        lex.add_type("integer", TypeName::TypeInteger);
        lex.add_type("floating", TypeName::TypeFloating);
        lex.add_type("boolean", TypeName::TypeBoolean);
        lex.add_type("object", TypeName::TypeObject);
        
        lex.add("true", Token::Boolean(true));
        lex.add("false", Token::Boolean(false));

        return lex;
    }

    fn add(&mut self, key: &str, value: Token) {
        let ident = is_ident_string(key);
        self.trie.set(key, TokenPack::new(value, ident));
    }

    fn add_type(&mut self, key: &str, value: TypeName) {
        self.add(key, Token::Type(value));
    }

    fn add_special(&mut self, key: &str, value: SpecialToken) {
        self.add(key, Token::Special(value));
    }
    
    pub fn run(&self, data: &str) -> Result<Vec<TokenInfo>, String> {
        let mut result = Vec::new();
        result.push(TokenInfo::new(Token::None, (0, 0)));
        let mut scanner = Scanner::new(data);
        
        while !scanner.eol() {
            skip_spaces(&mut scanner);
            let backup = scanner.backup();
            let pos = scanner.position();
            let next = self.trie.get(&mut scanner);
    
            match next {
                Some(expr) => {
                    let top = scanner.top();
                    if expr.0.possible_ident && (is_ident(top) || top.is_digit(10)) {
                        let mut ival = String::from(&backup.get()[0..expr.1]);
                        ival.push_str(&scan_ident(&mut scanner));
                        result.push(TokenInfo::new(Token::Ident(ival), pos));
                    } else {
                        if Token::is_special(SpecialToken::Hash)(&expr.0.value) {
                            scanner.advance_while(|c| { c != '\n' });
                        } else {
                            result.push(TokenInfo::new(expr.0.value.clone(), pos));
                        }
                    }
                },
                None => {
                    if scanner.top().is_digit(10) {
                        let num = scan_number(&mut scanner);
                        match num {
                            Number::Integer(i) => result.push(TokenInfo::new(Token::Integer(i), pos)),
                            Number::Floating(f) => result.push(TokenInfo::new(Token::Floating(f), pos)),
                        }
                    } else if is_ident(scanner.top()) {
                        let ident = scan_ident(&mut scanner);
                        result.push(TokenInfo::new(Token::Ident(ident), pos));
                    } else if scanner.top() == '"' {
                        scanner.advance();
                        let svalue = scan_string(&mut scanner, "\"");
                        result.push(TokenInfo::new(Token::String(svalue), pos));
                    } else if !scanner.eol() {
                        return Err(format!("Unexpected character '{}' at {}:{}", 
                            scanner.top(), pos.0, pos.1));
                    }
                },
            }
        }
        return Ok(result);
    }
}

