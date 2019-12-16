#![allow(unused)]

use std::collections::HashMap;
use std::ops::{Add, Sub};

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

pub struct Options {}
impl Options {
    pub fn new() -> Options {
        Options{}
    }
}

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
        !self.values.iter().find(|&x| *val == *x).is_none()
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

impl<T> PossibleArray<T> {
    pub fn add_value(&mut self, value: T) {
        match self {
            PossibleArray::Value(val) => { *val = value },
            PossibleArray::Array(vec) => { vec.push(value) },
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            PossibleArray::Value(_) => false,
            PossibleArray::Array(_) => true,
        }
    }
} 

pub trait ObjectBase {
    fn new() -> Self;
    fn set_name(&mut self, name: &str);
    fn is_array(&self) -> bool;
    fn set_array(&mut self);
}

pub struct StringType {
    value: PossibleArray<String>,
    enum_values: Option<Enum<String>>,
    name: String,
    opts: Options,
}

impl ObjectBase for StringType {
    fn new() -> StringType {
        StringType {
            value: PossibleArray::Value(String::new()),
            enum_values: None,
            name: String::new(),
            opts: Options::new(),
        }
    }

    fn set_name(&mut self, value: &str) {
        self.name = String::from(value);
    }

    fn is_array(&self) -> bool {
        match &self.value {
            PossibleArray::Array(_) => true,
            _ => false,
        }
    }
    fn set_array(&mut self) {
        self.value = PossibleArray::Array(Vec::new())
    }
}

impl StringType {

    pub fn check_enum(&self, val: &String) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
    }

    pub fn add_value(&mut self, value: &str) {
        self.value.add_value(String::from(value));
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

pub trait Numeric: Copy + 
                PartialOrd + 
                PartialEq + 
                std::string::ToString {
    fn zero() -> Self;
    fn name() -> &'static str;
}

impl Numeric for i64 {
    fn zero() -> Self {
        0 as Self
    }
    fn name() -> &'static str {
        "integer"
    }
}

impl Numeric for f64 {
    fn zero() -> Self {
        0.0 as Self
    }
    fn name() -> &'static str {
        "floating"
    }
}

pub struct NumberType<T> {
    value: PossibleArray<T>,
    min_max: Interval<T>,
    enum_values: Option<Enum<T>>,
    name: String,
    opts: Options,
}

impl<T> ObjectBase for NumberType<T> where T: Numeric { 
    fn new() -> NumberType<T> {
        NumberType {
            value: PossibleArray::Value(T::zero()),
            min_max: Interval::none(),
            enum_values: None,
            name: String::new(),
            opts: Options::new(),
        }
    }
    
    fn set_name(&mut self, value: &str) {
        self.name = String::from(value);
    }

    fn is_array(&self) -> bool {
        match &self.value {
            PossibleArray::Array(_) => true,
            _ => false,
        }
    }

    fn set_array(&mut self) {
        self.value = PossibleArray::Array(Vec::new())
    }
}

impl <T> NumberType<T> where T: Numeric {

    pub fn add_value(&mut self, value: T) {
        self.value.add_value(value);
    }

    pub fn check_enum(&self, val: T) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
    }

    pub fn check_minmax(&self, val: T) -> bool {
        self.min_max.check(val)
    }

    pub fn to_string(&self) -> String {
        let mut res = String::from(T::name());
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[] ");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                res.push_str(" ");
                *val == T::zero()
            },
        };
        res.push_str(&self.name);
        if !empty {
            res.push_str(" = ");
            match &self.value {
                PossibleArray::Array(arr) => {
                    res.push_str("[");
                    res.push_str(&utils::string_join(&arr, ", "));
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

pub type IntegerType = NumberType<i64>;
pub type FloatingType = NumberType<f64>;

pub struct BooleanType {
    value: PossibleArray<bool>,
    name: String,
    opts: Options,
}

impl BooleanType {
    pub fn new() -> BooleanType {
        BooleanType{
            value: PossibleArray::Value(false),
            name: String::new(),
            opts: Options::new(),
        }
    }
    pub fn add_value(&mut self, value: bool) {
        self.value.add_value(value);
    }

    pub fn to_string(&self) -> String {
        let mut res = String::from("boolean");
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[] ");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                res.push_str(" ");
                !val
            },
        };
        res.push_str(&self.name);
        if !empty {
            res.push_str(" = ");
            match &self.value {
                PossibleArray::Array(arr) => {
                    res.push_str("["); 
                    res.push_str(&utils::string_join(&arr, ", "));
                    res.push_str("]");
                },
                PossibleArray::Value(val) => {
                    res.push_str(&format!("{}", val));
                },
            }
        }
        return res;
    }        
}

impl ObjectBase for BooleanType {
    fn new() -> Self {
        BooleanType::new()
    }
    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    fn is_array(&self) -> bool {
        self.value.is_array()
    }
    fn set_array(&mut self) {
        self.value = PossibleArray::Array(Vec::new())
    }
}

pub struct ObjectType {
    value: PossibleArray<HashMap<String, Element>>,
    name: String,
    opts: Options,
}

impl ObjectType {
    pub fn new() -> ObjectType {
        value: PossibleArray::Value(HashMap::new()),
        name: String::new(),
        opts: Options::new(),
    }
    
}

impl ObjectBase for ObjectType {
    fn new() -> Self {
        ObjectType::new()
    }
    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    fn is_array(&self) -> bool {
        self.value.is_array()
    }
    fn set_array(&mut self) {
        self.value = PossibleArray::Array(Vec::new());
    }
}

pub enum Element {
    None,
    Str(StringType),
    Integer(IntegerType),
    Floating(FloatingType),
    Boolean(BooleanType),
    Object(ObjectType),
}
