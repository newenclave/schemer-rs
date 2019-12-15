#![allow(unused)]

mod schemer;
use schemer::lexer::Lexer;
use schemer::parser::Parser;
use schemer::tokens::{Token, SpecialToken};


fn main() {

    let val = "
string[] data = [\"bla bla\", \"1\", \"2\"]        
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
    
    let sss = pars.parse_string();

    println!("{}", sss.to_string());

}
