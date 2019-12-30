use std::collections::HashMap;
use std::env;
use std::fs;
mod schemer;
use schemer::objects::{FieldType};
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::to_schemer::{field_to_string};
use schemer::to_json::{to_json_values, to_json_schema};

fn show_in_json_value(value: &FieldType, shift: usize) {
    println!("{}", to_json_values(value, shift));
}

fn show_in_json_schema(value: &FieldType, shift: usize) {
    println!("{}", to_json_schema(value, shift));
}

fn show_in_schemer(value: &FieldType, shift: usize) {
    println!("{}", field_to_string(value, shift));
}

fn parse_format(obj: &str, call: &'static dyn Fn(&FieldType, usize), shift: usize) {
    let lex = Lexer::new();
    let vec = lex.run(obj);
    let mut pars = Parser::new(Vec::new());
    match vec {
        Err(expr) => eprintln!("Parsing error: {}", expr),
        Ok(v) => {
            pars = Parser::new(v);
        },
    };
    
    match &pars.parse_field() {
        Ok(val) => call(val, shift),
        Err(err) => println!("Parser error: {}", err.msg()),
    };
}

fn main() {

    let mut calls: HashMap<String, &'static dyn Fn(&FieldType, usize)> = HashMap::new();
    calls.insert("json_value".to_string(), &show_in_json_value);
    calls.insert("json_schema".to_string(), &show_in_json_schema);
    calls.insert("schemer".to_string(), &show_in_schemer);

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let call_name = if args.len() > 2 { &args[2] } else { "schemer" };
        let call = match calls.get(call_name) {
            Some(func) => *func,
            None => &show_in_schemer,
        };
        let mut def_shift: usize = 2;
        if args.len() >= 4 {
            def_shift = match args[3].parse::<usize>() {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("Invalid shift value '{}'. Error: {}", args[2], err);
                    2
                },
            }
        }
        let test_object = fs::read_to_string(&args[1]);
        match &test_object {
            Ok(obj) => {
                parse_format(obj, call, def_shift);
            },
            Err(err) => {
                eprintln!("reading file {} error. {}", args[1], err);
            },
        }
    } else {
        let v = "
        main: object {s: string enum {\"one\",\"two\",\"three\"} = \"one\",i: integer enum {1,3,5,7,9} = 3,f: floating enum {0.5,1,1.5} = 0.5}
        ".to_owned();

      parse_format(&v, &show_in_schemer, 2);
      // parse_format(&v, &|fld|{
      //     println!("{}", element_format(fld.value(), 2));
      // });
      eprintln!("Use: schemer-rs <path_to_scheme_file>")
    }
}
