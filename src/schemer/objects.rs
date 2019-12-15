#![allow(unused)]

use std::collections::HashMap;

pub enum PossibleArray<T> {
    Value(T),
    Array(Vec<T>),
}

pub struct StringType {
    prototype: Box<Option<StringType>>,
    value: PossibleArray<String>,
    name: String,
}

impl StringType {
    pub fn new() -> StringType {
        StringType {
            prototype: Box::new(None),
            value: PossibleArray::Value(String::new()),
            name: String::new(),        
        }
    }
    pub fn set_name(&mut self, value: &str) {
        self.name = String::from(value);
    }

    pub fn is_array(&self) -> bool {
        match &self.value {
            PossibleArray::Array(_) => true,
            _ => false,
        }
    }

    pub fn set_array(&mut self, values: Vec<String>) {
        self.value = PossibleArray::Array(values)
    }

    pub fn add_value(&mut self, value: &str) {
        match &mut self.value {
            PossibleArray::Value(val) => { *val = String::from(value) },
            PossibleArray::Array(vec) => { vec.push(String::from(value)) },
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::from("string");
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[] ");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                res.push_str(" ");
                val.len() == 0
            },
        };
        res.push_str(&self.name);
        if !empty {
            res.push_str(" = ");
            match &self.value {
                PossibleArray::Array(arr) => {
                    res.push_str("[\"");
                    res.push_str(&arr.join("\", \""));
                    res.push_str("\"]");
                },
                PossibleArray::Value(val) => {
                    res.push_str(&format!("\"{}\"", val));
                },
            }
        }
        return res;
    }
}

struct IntegerType {}
struct FloatingType {}
struct BooleanType {}
struct ObjectType {}

struct IntegerTypeInfo {}
struct FloatingTypeInfo {}
struct BooleanTypeInfo {}
struct ObjectTypeInfo {}

enum Element {
    None,
    Str(StringType),
    Integer(IntegerType),
    Floating(FloatingType),
    Boolean(BooleanType),
    Object(ObjectType),
}

