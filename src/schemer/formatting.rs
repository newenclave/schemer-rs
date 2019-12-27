#![allow(unused)]

use super::objects::*;
use super::object_base::*;
use super::helpers::*;

struct Formatting {
    new_line: &'static str,
    shift: String,
}

impl Formatting {
    fn new(shift: usize) -> Formatting {
        Formatting {
            new_line: if shift == 0 { "" } else { "\n" },
            shift: if shift == 0 { String::new() } else { " ".repeat(shift) },
        }
    }

    fn sh(&self, shift: usize) -> String {
        self.shift.repeat(shift)
    }

    fn format_array(&self, arr: &Vec<String>, shift: usize) -> String {
        format!("{}{}{}{}{}", self.new_line, self.sh(shift + 1), 
            arr.join(&format!(",{}{}", self.new_line, self.sh(shift + 1))), 
            self.new_line, self.sh(shift))
    }
}

trait ValueToString {
    fn convert(&self) -> String;
}

impl ValueToString for bool {
    fn convert(&self) -> String {
        String::from(if *self { "true" } else { "false" })
    }
}

impl ValueToString for i64 {
    fn convert(&self) -> String {
        self.to_string()
    }
}

impl ValueToString for f64 {
    fn convert(&self) -> String {
        self.to_string()
    }
}

impl ValueToString for String {
    fn convert(&self) -> String {
        format!("\"{}\"", self)
    }
}

fn value_format<T: Clone + ValueToString>(value: &PossibleArray<T>, shift: usize, start_shift: usize) -> String {
    let format = Formatting::new(shift);
    match value {
        PossibleArray::Value(val) => { (val as &dyn ValueToString).convert() },
        PossibleArray::Array(arr) => {
            format!("[{}]",
                format.format_array(&arr.iter().map(|v| (v as &dyn ValueToString).convert()).collect::<Vec<String>>(), start_shift)
            )
        },
    }
}

fn object_format(obj: &ObjectType, shift: usize, start_shift: usize) -> String {
    let format = Formatting::new(shift);
    match obj.value() {
        PossibleArray::Value(val) => {
            let str_value = match &**val {
                Some(unboxed) => {
                    unboxed.fields()
                },
                None => obj.fields(),
            }.iter().map(|(k, v)| {
                format!("\"{}\": {}", k, element_format(v.value(), shift, start_shift + 1))
            }).collect::<Vec<String>>();
            if str_value.len() == 0 {
                "{}".to_string()
            } else {
                format!("{{{}}}", format.format_array(&str_value, start_shift))
            }
        },
        PossibleArray::Array(arr) => {
            let str_value = (**arr)
            .iter().map(|x| {
                match &**x {
                    Some(field) => object_format(field, shift, start_shift + 1),
                    None => String::new(),
                }
            }).collect::<Vec<String>>();
            if str_value.len() == 0 {
                "[]".to_string()
            } else {
                format!("[{}]", format.format_array(&str_value, start_shift))
            }
        },
        _ => String::new()
    }
}

fn any_format(any: &AnyType, shift: usize, start_shift: usize) -> String {
    let format = Formatting::new(shift);
    match any.value() {
        PossibleArray::Array(arr) => {
            let str_values = (**arr)
            .iter().map(|x| {
                match &**x {
                    Some(field) => element_format(field, shift, start_shift + 1),
                    None => "null".to_string(),
                }
            }).collect::<Vec<String>>();
            if str_values.len() == 0 {
                "[]".to_string()
            } else {
                format!("[{}]", format.format_array(&str_values, start_shift))
            }
        },
        PossibleArray::Value(val) => {
            match &**val {
                Some(unboxed) => {
                    element_format(unboxed, shift, start_shift + 1)
                },
                None => "null".to_string(),
            }
        },
    }
}


pub fn element_format(element: &Element, shift: usize, start_shift: usize) -> String {
    match element {
        Element::Boolean(v) => { value_format(v.value(), shift, start_shift) },
        Element::String(v) => { value_format(v.value(), shift, start_shift) },
        Element::Integer(v) => { value_format(v.value(), shift, start_shift) },
        Element::Floating(v) => { value_format(v.value(), shift, start_shift) },
        Element::Object(v) => { object_format(v, shift, start_shift) },
        Element::Any(v) => { any_format(v, shift, start_shift) },
        Element::None => "".to_string(),
    }
}
