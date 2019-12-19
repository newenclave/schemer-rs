use std::collections::HashMap;
use std::env;
use std::fs;
mod schemer;
use schemer::objects::{FieldType};
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::to_schemer::{field_to_string};
use schemer::to_json::{to_json_values, to_json_schema};

fn show_in_json_value(value: &FieldType) {
    println!("{}", to_json_values(value));
}

fn show_in_json_schema(value: &FieldType) {
    println!("{}", to_json_schema(value));
}

fn show_in_schemer(value: &FieldType) {
    println!("{}", field_to_string(value));
}

fn parse_format(obj: &str, call: &'static dyn Fn(&FieldType)) {
    let lex = Lexer::new();
    let vec = lex.run(obj);
    let mut pars = Parser::new(Vec::new());
    match vec {
        Err(expr) => eprintln!("Parsing error: {}", expr),
        Ok(v) => {
            pars = Parser::new(v);
        },
    };
    
    let sss = pars.parse_field();
    call(&sss);
}

fn main() {

    let mut calls: HashMap<String, &'static dyn Fn(&FieldType)> = HashMap::new();
    calls.insert("json_value".to_string(), &show_in_json_value);
    calls.insert("json_schema".to_string(), &show_in_json_schema);
    calls.insert("schemer".to_string(), &show_in_schemer);

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let call_name = if args.len() > 2 { &args[2] } else { "schemer" };
        let call = match calls.get(call_name) {
            Some(func) => *func,
            None => &show_in_schemer,
        };
        let test_object = fs::read_to_string(&args[1]);
        match &test_object {
            Ok(obj) => {
                parse_format(obj, call);
            },
            Err(err) => {
                eprintln!("reading file {} error. {}", args[1], err);
            },
        }
    } else {
        let v = "
        main: object {
            i: integer
        } = {
            b = false,
            d = 0.9
        }        
        ".to_owned();
        parse_format(&v, &show_in_schemer);
        eprintln!("Use: schemer-rs <path_to_scheme_file>")
    }
}
