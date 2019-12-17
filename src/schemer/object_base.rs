#![allow(unused)]

use super::objects::*;
use super::helpers::*;


mod utils {
    pub fn string_join<T: std::string::ToString>(vals: &Vec<T>, sep: &str) -> String {
        let mut first = true;
        let mut res = String::new();

        for v in vals.iter() {
            if first {
                first = false;
            } else {
                res.push_str(sep);
            }
            res.push_str(&v.to_string());
        }
        return res;
    }
}

pub trait ObjectBase {
    fn create() -> Self;
    fn is_array(&self) -> bool;
    fn is_default(&self) -> bool;
    fn set_array(&mut self);
    fn value_to_string(&self) -> String;
}

impl ObjectBase for StringType {
    fn create() -> Self {
        StringType::new()
    }
    fn is_array(&self) -> bool {
        match self.value() {
            PossibleArray::Array(_) => true,
            _ => false,
        }
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => v.len() == 0,
            PossibleArray::Array(v) => v.len() == 0,
        }        
    }    
    fn set_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()))
    }

    fn value_to_string(&self) -> String {
        let mut res = String::new();
        match self.value() {
            PossibleArray::Array(arr) => {
                res.push_str("[\"");
                res.push_str(&arr.join("\", \""));
                res.push_str("\"]");
            },
            PossibleArray::Value(val) => {
                res.push_str(&format!("\"{}\"", val));
            },
        }
        return res;
    }
}

impl ObjectBase for BooleanType {
    fn create() -> Self {
        BooleanType::new()
    }
    fn is_array(&self) -> bool {
        self.value().is_array()
    }
    fn set_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()));
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => !*v,
            PossibleArray::Array(v) => v.len() == 0,
        }
    }

    fn value_to_string(&self) -> String {
        let mut res = String::new();
        match self.value() {
            PossibleArray::Array(arr) => {
                res.push_str("["); 
                res.push_str(&utils::string_join(&arr, ", "));
                res.push_str("]");
            },
            PossibleArray::Value(val) => {
                res.push_str(&format!("{}", val));
            },
        }
        return res;
    }
}

impl ObjectBase for ObjectType {
    fn create() -> Self {
        ObjectType::new()
    }
    fn is_array(&self) -> bool {
        self.value().is_array()
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => false,
            PossibleArray::Array(v) => v.len() == 0,
        }
    }    
    fn set_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()));
    }

    fn value_to_string(&self) -> String {
        let mut res = String::new();
        match self.value() {
            PossibleArray::Array(arr) => {
            let values: Vec<String> = (**arr)
                .iter()
                .map(|x| {
                    match &**x {
                        Some(expr) => expr.value_to_string(),
                        None => String::new(),
                    }
                }).collect();
                res.push_str("[");
                res.push_str(&values.join(", "));
                res.push_str("]");
            },
            PossibleArray::Value(val) => {
                res.push_str("{");
                let field_info: Vec<String> = self.fields()
                    .iter()
                    .map(|(k, v)| {
                        String::from(k) + " = " + &v.value().value_to_string()
                    }
                ).collect();
                res.push_str(&field_info.join(", "));
                res.push_str("}");
            },
        }
        return res;
    }
}

impl<T> ObjectBase for NumberType<T> where T: Numeric, T: Clone { 
    fn create() -> NumberType<T> {
        NumberType::new()
    }
    fn is_array(&self) -> bool {
        self.value().is_array()
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => *v == T::zero(),
            PossibleArray::Array(v) => v.len() == 0,
        }        
    }

    fn set_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()));
    }

    fn value_to_string(&self) -> String {
        let mut res = String::new();
        match self.value() {
            PossibleArray::Array(arr) => {
                res.push_str("[");
                res.push_str(&utils::string_join(&arr, ", "));
                res.push_str("]"); 
            },
            PossibleArray::Value(val) => {
                res.push_str(&format!("{:.8}", &val.to_string()));
            },
        }
        return res;
    }
}
