#![allow(unused)]

mod schemer;
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::tokens::{Token, SpecialToken};
use schemer::objects::{Element};


fn main() {

    let _val = "
main: object[] {
    data: string = \"hello!\"
    value: integer = 100
    flag: boolean = false
    num: floating[] = [0.009, 1, 100.5]
    inside: object {
        repeat: integer 1..100 = 50
        flop: floating 1..100 = 50
    }
    empty: object[] {} = [{}, {}, {}]
}
    ".to_string();

    let val = "
main: object[] {
    data: integer = 10
    mix: object[] {
        min: integer = 0
    }
   } = [{mix = [{min = 1}, {min = 2}, {min = 3}]}]
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

    println!("{}", sss.to_string(0));
}
