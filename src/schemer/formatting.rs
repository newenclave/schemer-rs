#![allow(unused)]

use super::objects::*;
use super::object_base::*;
use super::helpers::*;

pub struct Formatting {
    new_line: &'static str,
    shift: String,
}

impl Formatting {
    pub fn new(shift: usize) -> Formatting {
        Formatting {
            new_line: if shift == 0 { "" } else { "\n" },
            shift: if shift == 0 { String::new() } else { " ".repeat(shift) },
        }
    }

    fn sh(&self, shift: usize) -> String {
        self.shift.repeat(shift)
    }

    fn nl_sh(&self, shift: usize) -> String {
        format!(",{}{}", self.new_line, self.sh(shift))
    }

    pub fn format_array(&self, arr: &Vec<String>, shift: usize) -> String {
        if arr.len() > 0 {
            format!("{}{}{}{}{}", self.new_line, self.sh(shift + 1), 
            arr.join(&self.nl_sh(shift + 1)), 
            self.new_line, self.sh(shift))
        } else {
            String::new()
        }
    }

    pub fn format_t_array<T: format::ValueToString>(&self, arr: &Vec<T>, shift: usize) -> String {
        self.format_array(&arr.iter().map(|v| v.convert()).collect::<Vec<String>>(), shift)
    }

    pub fn format_value<T: format::ValueToString>(&self, val: &T) -> String {
        val.convert()
    }
}

pub mod format {

    use super::{
        Formatting,
        PossibleArray,
        ObjectType,
        AnyType, 
        Element
    };

    pub trait ValueToString {
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

    fn value_format<T: Clone + ValueToString>(value: &PossibleArray<T>, format: &Formatting, shift: usize) -> String {
        match value {
            PossibleArray::Value(val) => { (val as &dyn ValueToString).convert() },
            PossibleArray::Array(arr) => {
                format!("[{}]",
                    format.format_array(&arr.iter().map(|v| {
                        (v as &dyn ValueToString).convert()
                    }).collect::<Vec<String>>(), shift)
                )
            },
        }
    }

    fn object_format(obj: &ObjectType, format: &Formatting, shift: usize) -> String {
        match obj.value() {
            PossibleArray::Value(val) => {
                let str_value = match &**val {
                    Some(unboxed) => {
                        unboxed.fields()
                    },
                    None => obj.fields(),
                }.iter().map(|(k, v)| {
                    format!("\"{}\": {}", k, element_format_impl(v.value(), format, shift + 1))
                }).collect::<Vec<String>>();
                format!("{{{}}}", format.format_array(&str_value, shift))
            },
            PossibleArray::Array(arr) => {
                let str_value = (**arr)
                .iter().map(|x| {
                    match &**x {
                        Some(field) => object_format(field, format, shift + 1),
                        None => String::new(),
                    }
                }).collect::<Vec<String>>();
                format!("[{}]", format.format_array(&str_value, shift))
            },
            _ => String::new()
        }
    }

    fn any_format(any: &AnyType, format: &Formatting, shift: usize) -> String {
        match any.value() {
            PossibleArray::Array(arr) => {
                let str_values = (**arr)
                .iter().map(|x| {
                    match &**x {
                        Some(field) => element_format_impl(field, format, shift + 1),
                        None => "null".to_string(),
                    }
                }).collect::<Vec<String>>();
                format!("[{}]", format.format_array(&str_values, shift))
            },
            PossibleArray::Value(val) => {
                match &**val {
                    Some(unboxed) => {
                        element_format_impl(unboxed, format, shift + 1)
                    },
                    None => "null".to_string(),
                }
            },
        }
    }

    pub fn element_format_impl(element: &Element, format: &Formatting, shift: usize) -> String {
        match element {
            Element::Boolean(v) => { value_format(v.value(), format, shift) },
            Element::String(v) => { value_format(v.value(), format, shift) },
            Element::Integer(v) => { value_format(v.value(), format, shift) },
            Element::Floating(v) => { value_format(v.value(), format, shift) },
            Element::Object(v) => { object_format(v, format, shift) },
            Element::Any(v) => { any_format(v, format, shift) },
            Element::None => "".to_string(),
        }
    }
}

pub fn element_format(element: &Element, shift: usize) -> String {
    let format = Formatting::new(shift);
    format::element_format_impl(element, &format, 0)
}
