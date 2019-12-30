#![allow(unused)]
use std::collections::HashMap;
use std::env;
use std::fs;
mod schemer;
use schemer::objects::{Module};
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::to_schemer::{module_to_string};
use schemer::to_json::{to_json_schema, to_json_values};

fn show_in_json_value(value: &Module, shift: usize, _: &str) {
    println!("{}", to_json_values(value, shift))
}

fn show_in_json_schema(value: &Module, shift: usize, _: &str) {
    println!("{}", to_json_schema(value, shift));
}

fn show_in_schemer(value: &Module, shift: usize, _: &str) {
    println!("{}", module_to_string(value, shift));
}

fn parse_format(obj: &str, call: &'static dyn Fn(&Module, usize, &str), shift: usize, root_name: &str) {
    let lex = Lexer::new();
    let vec = lex.run(obj);
    let mut pars = Parser::new(Vec::new());
    match vec {
        Err(expr) => eprintln!("Parsing error: {}", expr),
        Ok(v) => {
            pars = Parser::new(v);
        },
    };
    
    match &pars.parse_module() {
        Ok(val) => call(val, shift, root_name),
        Err(err) => println!("Parser error: {}", err.msg()),
    };
}

fn main() {

    let mut calls: HashMap<String, &'static dyn Fn(&Module, usize, &str)> = HashMap::new();
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
        let mut root_name = "main";
        if args.len() >= 4 {
            root_name = &args[3];
            if args.len() >= 5 {
                def_shift = match args[4].parse::<usize>() {
                  Ok(val) => val,
                  Err(err) => {
                      eprintln!("Invalid shift value '{}'. Error: {}", args[4], err);
                      return ();
                  },
              }
            }
        }
        let test_object = fs::read_to_string(&args[1]);
        match &test_object {
            Ok(obj) => {
                parse_format(obj, call, def_shift, root_name);
            },
            Err(err) => {
                eprintln!("reading file {} error. {}", args[1], err);
            },
        }
    } else {
        let v = "
        mod a.b.c
        main: object {
          i: object {
            a: any = {
              b: true
            }
          } = {
            a: { b: [1, 2, 3] },
          }
        }
        root: object {
          o: object {}
        }
        ".to_owned();

        let lex = Lexer::new();
        let vec = lex.run(&v);
        let mut pars = Parser::new(Vec::new());
        match vec {
            Err(expr) => eprintln!("Parsing error: {}", expr),
            Ok(v) => {
                pars = Parser::new(v);
            },
        };
        match &pars.parse_module() {
            Ok(val) => {
                println!("mod: {}", module_to_string(val, 2));
            }
            Err(err) => println!("Parser error: {}", err.msg()),
        };
        eprintln!("Use: schemer-rs <path_to_scheme_file>")
    } 
}
