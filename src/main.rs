#![allow(unused)]

mod schemer;
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::tokens::{Token, SpecialToken};
use schemer::objects::{Element};


fn main() {

    let val = "
data: object = {
    name: string
    data: floating
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
    
    let sss = pars.parse_field();

    match sss.value() {
        Element::None => {},
        Element::Str(v) => { println!("{}", v.to_string()); },
        Element::Integer(v) => { println!("{}", v.to_string()); },
        Element::Floating(v) => { println!("{}", v.to_string()); },
        Element::Boolean(v) => { println!("{}", v.to_string()); },
        Element::Object(v) => { println!("obj"); },
    }
}
