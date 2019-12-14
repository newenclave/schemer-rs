#![allow(unused)]

mod schemer;
use schemer::lexer::Lexer;
use schemer::parser::Parser;
use schemer::tokens::Token;

fn main() {

    let val = "
        readonly object[] ready {
            string s
            integer i
        }
    ".to_string();
    let lex = Lexer::new();
    let vec = lex.run(&val);

    let mut pars = Parser::new([].to_vec());

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
    }
}
