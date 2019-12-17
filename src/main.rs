#![allow(unused)]

mod schemer;
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::tokens::{Token, SpecialToken};
use schemer::objects::{Element};
use schemer::to_schemer::*;


fn main() {

    let val = "
    main: object[] {
        flag: boolean,
        data: string = \"hello!\",
        f: floating = 0.5,
        i: integer[] = [1, 2, 3, 4]
      } = [
        {
          flag: false,
          data: \"hello!\",
          f: 0.5,
          i: [1, 2, 3, 4]
        },
        {
          flag: false,
          data: \"hello!\",
          f: 0.5,
          i: [1, 2, 3, 4]
        },
        {
          flag: false,
          data: \"hello!\",
          f: 0.5,
          i: [1, 2, 3, 4]
        }
      ]
    ".to_owned();

    let _val = "
    object[] {
        int: object {
         int: object {
          int: object {
           int: object {
            int: object {
             itnt: boolean
            }
           }
          }
         }
        }
        name: string = \"123\"
        value: integer 1..100
       } = [{int = {int = {int = {int = {int = {itnt = false}}}}}, name = \"test name\", value = 50}, {int = {int = {int = {int = {int = {itnt = false}}}}}, name = \"123\", value = 0}, {int = {int = {int = {int = {int = {itnt = false}}}}}, name = \"123\", value = 0}]
    ".to_owned();
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

    println!("{}", fild_to_schemer_string(&sss));
}
