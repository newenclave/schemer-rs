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
        format!("[{}{}{}{}{}]", self.new_line, self.sh(shift + 1), 
            arr.join(", "), self.sh(shift), self.new_line)
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

fn value_format<T: Clone + ValueToString>(value: &PossibleArray<T>, shift: usize) -> String {
    let format = Formatting::new(shift);
    match value {
        PossibleArray::Value(val) => { (val as &dyn ValueToString).convert() },
        PossibleArray::Array(arr) => {
            format.format_array(&arr.iter().map(|v| (v as &dyn ValueToString).convert()).collect::<Vec<String>>(), 0)
        },
    }
}

// fn object_format(obj: &ObjectType, shift: usize) {
//     let format = Formatting::new(shift);
//     match obj {
//         PossibleArray::Value(val) => {
//             let str_value = match &**val {
//                 Some(unboxed) => {
//                     unboxed.fields()
//                 },
//                 None => obj.fields(),
//             }.iter().map(|(k, v)| {
//                 to_json_values_impl(k, v.value(), shift + 1)
//             }).collect::<Vec<String>>().join(",\n");
//             if str_value.len() == 0 {
//                 "{}".to_string()
//             } else {
//                 format!("{{\n{}\n{}}}", &str_value, utils::sh(shift))
//             }
//         },
//         PossibleArray::Array(arr) => {
//             let values = (**arr)
//             .iter().map(|x| {
//                 match &**x {
//                     Some(field) => field.value_to_json(shift + 1),
//                     None => String::new(),
//                 }
//             }).collect::<Vec<String>>().join(", ");
//             if values.len() == 0 {
//                 "[]".to_string()
//             } else {
//                 format!("[\n{}{}\n{}]", utils::sh(shift + 1), values, utils::sh(shift))
//             }
//         },
//     }
// }

pub fn element_format(element: &Element, shift: usize) -> String {
    match element {
        Element::Boolean(v) => { value_format(v.value(), shift) },
        Element::String(v) => { value_format(v.value(), shift) },
        Element::Integer(v) => { value_format(v.value(), shift) },
        Element::Floating(v) => { value_format(v.value(), shift) },
        // Element::Object(v) => { call_value_to_json(v, shift) },
        // Element::Any(v) => { call_value_to_json(v, shift) },
        // Element::None => "".to_string(),
        _ => String::new()
    }
}