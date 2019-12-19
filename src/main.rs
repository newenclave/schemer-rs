//#![allow(unused)]

use std::env;
use std::fs;
mod schemer;
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::to_schemer::{field_to_string};
use schemer::to_json::{to_json_values, to_json_schema};


fn parse_format(obj: &str, to: &str) {
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
    if to == "shcemer" {
        println!("{}", field_to_string(&sss));
    } else {
        println!("{}", to_json_schema(&sss));
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let test_object = fs::read_to_string(&args[1]);
        match &test_object {
            Ok(obj) => {
                parse_format(obj, "");
            },
            Err(err) => {
                println!("reading file {} error. {}", args[1], err);
            },
        }
    } else {
        let v = "
        main: any = {
            data: \"string value\",
            i: 1000,
            f: 0.5,
            a: true,
            b: false,
            n: null,
            aa: {}
        }
        ".to_owned();
        parse_format(&v, "");
        println!("Use: schemer-rs <path_to_scheme_file>")
    }
}
