//#![allow(unused)]

use std::env;
use std::fs;
mod schemer;
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::to_schemer::{field_to_string};

fn parse_format(obj: &str) {
    let lex = Lexer::new();
    let vec = lex.run(obj);
    let mut pars = Parser::new(Vec::new());
    match vec {
        Err(expr) => println!("Parsing error: {}", expr),
        Ok(v) => {
            pars = Parser::new(v);
        },
    };
    
    let sss = pars.parse_field();
    println!("looks like no panic:\n{}", field_to_string(&sss));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let test_object = fs::read_to_string(&args[1]);
        match &test_object {
            Ok(obj) => {
                parse_format(obj);
            },
            Err(err) => {
                println!("reading file {} error. {}", args[1], err);
            },
        }
    } else {
        let v = "
        main(aaa: [1, 4.5, \"help\", [1, 2, 3]]): object[] {
            i: integer 1..3 enum { 1, 2, 3 } = 1
            s: string enum {\"empty\"}
        }
        ".to_owned();
        parse_format(&v);
        println!("Use: schemer-rs <path_to_scheme_file>")
    }
}
