#![allow(unused)]

use super::objects::*;

trait ToSchemer {
    fn to_schemer_string(&self) -> String;
}

pub to_schemer_string<T: ToSchemer>(obj: &T) {
    obj.to_schemer_string()
}
