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

#[derive(Clone)]
pub struct Options {}
impl Options {
    pub fn new() -> Options {
        Options{}
    }
}

#[derive(Clone)]
struct Interval<T: Clone> {
    min_max: (Option<T>, Option<T>),
}

impl<T> Interval<T> where T: std::cmp::PartialOrd + Copy {
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
    pub fn set_min(&mut self, val: T) {
        self.min_max.0 = Some(val);
    }
    pub fn set_max(&mut self, val: T) {
        self.min_max.1 = Some(val);
    }

    pub fn has_min(&self) -> bool {
        !self.min_max.0.is_none()
    }
    pub fn has_max(&self) -> bool {
        !self.min_max.1.is_none()
    }
    pub fn has_minmax(&self) -> bool {
        self.has_min() || self.has_max()
    }
    pub fn min(&self, default: T) -> T {
        match &self.min_max.0 {
            Some(val) => *val,
            None => default,
        }
    }
    pub fn max(&self, default: T) -> T {
        match &self.min_max.1 {
            Some(val) => *val,
            None => default,
        }
    }
}

#[derive(Clone)]
struct Enum<T: Clone> {
    values: Vec<T>
}

impl<T> Enum<T> where T: std::cmp::PartialEq + Clone {
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

#[derive(Clone)]
enum PossibleArray<T: Clone> {
    Value(T),
    Array(Vec<T>),
}

impl<T> PossibleArray<T> where T: Clone {
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
    fn is_array(&self) -> bool;
    fn set_array(&mut self);
    fn clone(&self) -> Self;
    fn value_to_string(&self) -> String;
}

fn object_value_to_string<T: ObjectBase>(obj: &T) -> String {
    obj.value_to_string()
}

#[derive(Clone)]
pub struct StringType {
    value: PossibleArray<String>,
    enum_values: Option<Enum<String>>,
}

impl ObjectBase for StringType {
    fn new() -> StringType {
        StringType {
            value: PossibleArray::Value(String::new()),
            enum_values: None,
        }
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
    fn clone(&self) -> Self {
        StringType {
            value: self.value.clone(),
            enum_values: self.enum_values.clone(),
        }
    }
    fn value_to_string(&self) -> String {
        let mut res = String::new();
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
        return res;
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

pub trait Numeric: Copy + 
                Clone +
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

#[derive(Clone)]
pub struct NumberType<T: Clone> {
    value: PossibleArray<T>,
    min_max: Interval<T>,
    enum_values: Option<Enum<T>>,
}

impl<T> ObjectBase for NumberType<T> where T: Numeric, T: Clone { 
    fn new() -> NumberType<T> {
        NumberType {
            value: PossibleArray::Value(T::zero()),
            min_max: Interval::none(),
            enum_values: None,
        }
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
    fn clone(&self) -> Self {
        NumberType::new()
    }
    fn value_to_string(&self) -> String {
        let mut res = String::new();
        match &self.value {
            PossibleArray::Array(arr) => {
                res.push_str("[");
                res.push_str(&utils::string_join(&arr, ", "));
                res.push_str("]"); 
            },
            PossibleArray::Value(val) => {
                res.push_str(&format!("{:.4}", &val.to_string()));
            },
        }
        return res;
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

impl ObjectBase for BooleanType {
    fn new() -> Self {
        BooleanType::new()
    }
    fn is_array(&self) -> bool {
        self.value.is_array()
    }
    fn set_array(&mut self) {
        self.value = PossibleArray::Array(Vec::new());
    }
    fn clone(&self) -> Self {
        BooleanType::new()
    }
    fn value_to_string(&self) -> String {
        let mut res = String::new();
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

impl ObjectBase for ObjectType {
    fn new() -> Self {
        ObjectType::new()
    }
    fn is_array(&self) -> bool {
        self.value.is_array()
    }
    fn set_array(&mut self) {
        self.value = PossibleArray::Array(Vec::new());
    }
    fn clone(&self) -> Self {
        ObjectType::new()
    }
    fn value_to_string(&self) -> String {
        let mut res = String::new();
        match &self.value {
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
                let field_info: Vec<String> = self.fields
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

#[derive(Clone)]
pub enum Element {
    None,
    Str(StringType),
    Integer(IntegerType),
    Floating(FloatingType),
    Boolean(BooleanType),
    Object(ObjectType),
}

impl Element {
    pub fn clone(&self) -> Element {
        Element::None
    }
    fn value_to_string(&self) -> String {
        match &self {
            Element::None => "".to_string(),
            Element::Str(v) => { object_value_to_string(v) },
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
            Element::Str(v) => { v.to_string(start) },
            Element::Integer(v) => { v.to_string(start) },
            Element::Floating(v) => { v.to_string(start) },
            Element::Boolean(v) => { v.to_string(start) },
            Element::Object(v) => { v.to_string(start) },
        }
    }
}
