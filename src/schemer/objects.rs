#![allow(unused)]

use std::collections::HashMap;


pub struct Interval<T> {
    min_max: (Option<T>, Option<T>),
}

impl<T> Interval<T> where T: std::cmp::PartialOrd {
    pub fn new(min: Option<T>, max: Option<T>) -> Interval<T> {
        Interval {
            min_max: (min, max)
        }
    }
    pub fn none() -> Interval<T> {
        Interval {
            min_max: (None, None)
        }
    }
    pub fn check(&self, val: T) -> bool {
        return match &self.min_max.0 {
            Some(min_val) => *min_val <= val,
            None => true,
        } && match &self.min_max.1 {
            Some(max_val) => val <= *max_val,
            None => true,
        }
    }
}

pub struct Enum<T> {
    values: Vec<T>
}

impl<T> Enum<T> where T: std::cmp::PartialEq {
    pub fn new() -> Enum<T> {
        Enum {
            values: Vec::new(),       
        }
    }
    pub fn check(&self, val: &T) -> bool {
        match &self.values.iter().find(|&x| *val == *x) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn try_add(&mut self, val: T) -> bool {
        if self.check(&val) {
            false
        } else {
            self.values.push(val);
            true
        }
    }
}

pub enum PossibleArray<T> {
    Value(T),
    Array(Vec<T>),
}

pub struct StringType {
    value: PossibleArray<String>,
    enum_values: Option<Enum<String>>,
    name: String,
}

impl StringType {
    pub fn new() -> StringType {
        StringType {
            value: PossibleArray::Value(String::new()),
            enum_values: None,
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
    
    pub fn check_enum(&self, val: &String) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
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
                    res.push_str("[\"");               // [ 
                    res.push_str(&arr.join("\", \"")); //   ", "
                    res.push_str("\"]");               // ]
                },
                PossibleArray::Value(val) => {
                    res.push_str(&format!("\"{}\"", val));
                },
            }
        }
        return res;
    }
}


pub struct IntegerType {
    value: PossibleArray<i64>,
    min_max: Interval<i64>,
    enum_values: Option<Enum<i64>>,
    name: String,
}

impl IntegerType {
    pub fn new() -> IntegerType {
        IntegerType {
            value: PossibleArray::Value(0),
            min_max: Interval::none(),
            enum_values: None,
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

    pub fn set_array(&mut self, values: Vec<i64>) {
        self.value = PossibleArray::Array(values)
    }

    pub fn add_value(&mut self, value: i64) {
        match &mut self.value {
            PossibleArray::Value(val) => { *val = value },
            PossibleArray::Array(vec) => { vec.push(value) },
        }
    }

    pub fn check_enum(&self, val: i64) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
    }

    pub fn check_minmax(&self, val: i64) -> bool {
        self.min_max.check(val)
    }

    fn join(vals: &Vec<i64>, sep: &str) -> String {
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

    pub fn to_string(&self) -> String {
        let mut res = String::from("integer");
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[] ");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                res.push_str(" ");
                *val == 0
            },
        };
        res.push_str(&self.name);
        if !empty {
            res.push_str(" = ");
            match &self.value {
                PossibleArray::Array(arr) => {
                    res.push_str("[");
                    res.push_str(&IntegerType::join(arr, ", "));
                    res.push_str("]"); 
                },
                PossibleArray::Value(val) => {
                    res.push_str(&val.to_string());
                },
            }
        }
        return res;
    }
}

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

