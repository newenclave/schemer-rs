#![allow(unused)]

use std::collections::HashMap;
use std::ops::{Add, Sub};
use super::helpers::*;
use super::object_base::*;

#[derive(Clone)]
pub struct Options {}
impl Options {
    pub fn new() -> Options {
        Options{}
    }
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
}
