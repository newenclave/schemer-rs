#![allow(unused)]

use std::collections::HashMap;
use std::ops::{Add, Sub};
use super::helpers::*;
use super::object_base::*;

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

#[derive(Clone)]
pub struct Options {}
impl Options {
    pub fn new() -> Options {
        Options{}
    }
}

fn object_value_to_string<T: ObjectBase>(obj: &T) -> String {
    obj.value_to_string()
}

#[derive(Clone)]
pub struct StringType {
    value: PossibleArray<String>,
    enum_values: Option<Enum<String>>,
}


impl StringType {
    pub fn new() -> StringType {
        StringType {
            value: PossibleArray::Value(String::new()),
            enum_values: None,
        }
    }
    pub fn check_enum(&self, val: &String) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
    }

    pub fn value(&self) -> &PossibleArray<String> {
        &self.value
    }

    pub fn set_value(&mut self, val: PossibleArray<String>) {
        self.value = val;
    }

    pub fn add_value(&mut self, value: &str) {
        self.value.add_value(String::from(value));
    }

    pub fn to_string(&self, start: usize) -> String {
        let mut res = String::from("string");
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[]");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                val.len() == 0
            },
        };
        if !empty {
            res.push_str(" = ");
            res.push_str(&object_value_to_string(self));
        }
        return res;
    }
}

#[derive(Clone)]
pub struct NumberType<T: Clone> {
    value: PossibleArray<T>,
    min_max: Interval<T>,
    enum_values: Option<Enum<T>>,
}

impl <T> NumberType<T> where T: Numeric {
    pub fn new() -> NumberType<T> {
        NumberType {
            value: PossibleArray::Value(T::zero()),
            min_max: Interval::none(),
            enum_values: None,
        }
    }
    pub fn add_value(&mut self, value: T) {
        self.value.add_value(value);
    }

    pub fn check_enum(&self, val: T) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
    }

    pub fn value(&self) -> &PossibleArray<T> {
        &self.value
    }

    pub fn set_value(&mut self, val: PossibleArray<T>) {
        self.value = val;
    }

    pub fn interval(&self) -> &Interval<T> {
        &self.min_max
    }

    pub fn check_minmax(&self, val: T) -> bool {
        self.min_max.check(val)
    }

    pub fn set_min(&mut self, val: T) {
        self.min_max.set_min(val);
    }

    pub fn set_max(&mut self, val: T) {
        self.min_max.set_max(val);
    }

    pub fn to_string(&self, start: usize) -> String {
        let mut res = String::from(T::name());
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[]");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                *val == T::zero()
            },
        };
        if self.min_max.has_minmax() {
            res.push_str(" ");
            if self.min_max.has_min() {
                res.push_str(&self.min_max.min(T::zero()).to_string());
            }
            res.push_str("..");
            if self.min_max.has_max() {
                res.push_str(&self.min_max.max(T::zero()).to_string());
            }
        }
        if !empty {
            res.push_str(" = ");
            res.push_str(&object_value_to_string(self));
        }
        return res;
    }
}

pub type IntegerType = NumberType<i64>;
pub type FloatingType = NumberType<f64>;

#[derive(Clone)]
pub struct BooleanType {
    value: PossibleArray<bool>,
}

impl BooleanType {
    pub fn new() -> BooleanType {
        BooleanType{
            value: PossibleArray::Value(false),
        }
    }
    pub fn add_value(&mut self, value: bool) {
        self.value.add_value(value);
    }

    pub fn value(&self) -> &PossibleArray<bool> {
        &self.value
    }

    pub fn set_value(&mut self, val: PossibleArray<bool>) {
        self.value = val;
    }

    pub fn to_string(&self, start: usize) -> String {
        let mut res = String::from("boolean");
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[]");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                !val
            },
        };
        if !empty {
            res.push_str(" = ");
            res.push_str(&object_value_to_string(self));
        }
        return res;
    }
}


#[derive(Clone)]
pub struct ObjectType {
    value: PossibleArray<Box<Option<ObjectType>>>,
    name: String,
    opts: Options,
    fields: HashMap<String, FieldType>,
}

impl ObjectType {
    pub fn new() -> ObjectType {
        ObjectType {
            value: PossibleArray::Value(Box::new(None)),
            name: String::new(),
            opts: Options::new(),
            fields: HashMap::new(),
        }
    }
    pub fn has_field(&self, val: &str) -> bool {
        self.fields.contains_key(val)
    }

    pub fn add_field(&mut self, val: FieldType) {
        self.fields.insert(String::from(val.name()), val);
    }

    pub fn add_value(&mut self, value: ObjectType) {
        self.value.add_value(Box::new(Some(value)))
    }

    pub fn fields(&self) -> &HashMap<String, FieldType> {
        &self.fields
    }
    pub fn value(&self) -> &PossibleArray<Box<Option<ObjectType>>> {
        &self.value
    }
    pub fn set_value(&mut self, val: PossibleArray<Box<Option<ObjectType>>>) {
        self.value = val;
    } 

    pub fn get_field(&self, key: &str) -> Option<&FieldType> {
        self.fields.get(key)
    }

    pub fn set_fields(&mut self, new_values: HashMap<String, FieldType>) {
        self.fields = new_values;
    }

    pub fn clone_fields(&self) -> HashMap<String, FieldType> {
        self.fields.clone()
    }

    pub fn to_string(&self, start: usize) -> String {
        let mut res = String::from("object");
        let empty = match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[]");
                arr.len() == 0
            },
            PossibleArray::Value(val) => {
                match &*(*val) {
                    None => true,
                    _ => false,
                }
            },
        };
        res.push_str(" {\n");
        for (k, v) in self.fields.iter() {
            res.push_str(&" ".repeat(start + 1));
            res.push_str(&(k.to_owned() + ": "));
            res.push_str(&v.to_string(start + 1));
            res.push_str("\n");
        } 
        res.push_str(&" ".repeat(start));
        res.push_str("}");
        if !empty {
            res.push_str(" = ");
            res.push_str(&object_value_to_string(self));
        }
        return res
    }
}


#[derive(Clone)]
pub enum Element {
    None,
    String(StringType),
    Integer(IntegerType),
    Floating(FloatingType),
    Boolean(BooleanType),
    Object(ObjectType),
}

impl Element {
    pub fn clone(&self) -> Element {
        Element::None
    }
    pub fn value_to_string(&self) -> String {
        match &self {
            Element::None => "".to_string(),
            Element::String(v) => { object_value_to_string(v) },
            Element::Integer(v) => { object_value_to_string(v) },
            Element::Floating(v) => { object_value_to_string(v) },
            Element::Boolean(v) => { object_value_to_string(v) },
            Element::Object(v) => { object_value_to_string(v) },
        } 
    }
}

#[derive(Clone)]
pub struct FieldType {
    value: Element,
    name: String,
}

impl FieldType {
    pub fn new(name: String, value: Element) -> FieldType {
        FieldType{
            value: value,
            name: String::from(name),
        }
    }
    pub fn value<'a>(&'a self) -> &'a Element {
        return &self.value
    }
    pub fn name<'a>(&'a self) -> &'a str {
        return &self.name
    }

    pub fn to_string(&self, start: usize) -> String {
        match self.value() {
            Element::None => "".to_string(),
            Element::String(v) => { v.to_string(start) },
            Element::Integer(v) => { v.to_string(start) },
            Element::Floating(v) => { v.to_string(start) },
            Element::Boolean(v) => { v.to_string(start) },
            Element::Object(v) => { v.to_string(start) },
        }
    }
}
