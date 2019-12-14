#![allow(unused)]

mod schemer;
use schemer::lexer::Lexer;
use schemer::parser::Parser;
use schemer::tokens::{Token, SpecialToken};

fn main() {

    let val = "
        readonly object[] ready {
            string s
            integer i
        }
    ".to_string();
    let lex = Lexer::new();
    let vec = lex.run(&val);

    let mut pars = Parser::new(Vec::new());

    match &vec {
        Err(expr) => println!("Fail! {}", expr),
        Ok(v) => {
            pars = Parser::new(v.to_vec());
        },
    };
 
    while !pars.eof() {
        pars.advance();
        if pars.expect(&Token::is_ident()) {
            println!("found! {}", pars.current().to_string())
        }
        if pars.expect(&Token::is_special(SpecialToken::LBrace)) {
            println!("{}", pars.current().to_string())
        }
        if pars.expect(&Token::is_special(SpecialToken::RBrace)) {
            println!("{}", pars.current().to_string())
        }
    }
}
