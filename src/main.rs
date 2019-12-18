#![allow(unused)]

mod schemer;
use schemer::lexer::{Lexer};
use schemer::parser::Parser;
use schemer::tokens::{Token, SpecialToken};
use schemer::objects::{Element};
use schemer::to_schemer::{field_to_string};


fn main() {

    let _val = "
    main: object[] {
        o: object { 
            i: integer,
            f: floating
        } 
    } 
    ".to_owned();

    let val = "main: object[] {
        int: object {
          int: object {
            int: object {
              int: object {
                int: object {
                  itnt: boolean
                } = {
                  itnt: false
                }
              } = {
                int: {
                  itnt: false
                }
              }
            } = {
              int: {
                int: {
                  itnt: false
                }
              }
            }
          } = {
            int: {
              int: {
                int: {
                  itnt: false
                }
              }
            }
          }
        } = {
          int: {
            int: {
              int: {
                int: {
                  itnt: false
                }
              }
            }
          }
        },
        name: string = \"123\",
        value: integer
      } = [
      {
          int: {
            int: {
              int: {
                int: {
                  int: {
                    itnt: false
                  }
                }
              }
            }
          },
          name: \"test name\",
          value: 50
        }, {
          int: {
            int: {
              int: {
                int: {
                  int: {
                    itnt: false
                  }
                }
              }
            }
          },
          name: \"123\",
          value: 0
        }, {
          int: {
            int: {
              int: {
                int: {
                  int: {
                    itnt: false
                  }
                }
              }
            }
          },
          name: \"123\",
          value: 0
        }
      ]
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

    println!("{}", field_to_string(&sss));
}
