#![allow(unused)]

use super::objects::*;
use super::object_base::*;
use super::helpers::*;

static SHIFT: &'static str = "  ";

mod utils {
    pub fn string_join<T: std::string::ToString>(vals: &Vec<T>, sep: &str) -> String {
        vals.iter().map(|v|{
            v.to_string()
        }).collect::<Vec<String>>().join(sep)
    }
}

trait ToSchemerString {
    fn field_to(&self, shift: usize) -> String;
    fn value_to(&self, shift: usize) -> String;
}

fn cast<T: ToSchemerString>(val: &T) -> &T {
    val
}

impl ToSchemerString for BooleanType {
    fn field_to(&self, _: usize) -> String {
        format!("boolean{}", 
            if self.is_array() { "[]" } else { "" }
        )
    }
    fn value_to(&self, _: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                format!("[{}]", &utils::string_join(&arr, ", "))
            },
            PossibleArray::Value(val) => {
                format!("{}", val)
            },
        }
    }
}

impl ToSchemerString for StringType {
    fn field_to(&self, _: usize) -> String {
        format!("string{}", 
            if self.is_array() { "[]" } else { "" }
        )
    }
    fn value_to(&self, _: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                if arr.len() > 0 {
                    format!("[\"{}\"]", &utils::string_join(&arr, ", "))
                } else {
                    "[]".to_string()
                }
            },
            PossibleArray::Value(val) => {
                format!("\"{}\"", val)
            },
        }
    }
}

impl<T> ToSchemerString for NumberType<T> where T: Numeric, T: Clone {
    fn field_to(&self, _: usize) -> String {
        format!("{}{}", T::name(), 
            if self.is_array() { "[]" } else { "" }
        )
    }
    fn value_to(&self, _: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                format!("[{}]", &utils::string_join(&arr, ", "))
            },
            PossibleArray::Value(val) => {
                format!("{:.8}", &val.to_string())
            },
        }
    }
}

impl ToSchemerString for ObjectType {
    fn field_to(&self, shift: usize) -> String {
        let fields: Vec<String> = self.fields()
            .iter()
            .map(|(_, v)| {
                field_to_string_impl(v, shift + 1)
            }).collect();
        format!("object{} {{\n{}\n{}}}", 
            if self.is_array() { "[]" } else { "" },
            fields.join(",\n"),
            SHIFT.repeat(shift)
        )
    }
    fn value_to(&self, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
            let values: Vec<String> = (**arr)
                .iter()
                .map(|x| {
                    match &**x {
                        Some(field) => cast(field).value_to(shift + 1),
                        None => String::new(),
                    }
                }).collect();
                format!("[\n{}\n{}]", values.join(", "), SHIFT.repeat(shift))
            },
            PossibleArray::Value(val) => {
                let field_info: Vec<String> = self.fields()
                    .iter()
                    .map(|(k, v)| {
                        values_to_string(v, shift + 1)
                    }
                ).collect();
                format!("{{\n{}\n{}}}", &field_info.join(",\n"), SHIFT.repeat(shift))
            },
        }
    }
}

fn field_values_to_string<T: ObjectBase + ToSchemerString>(val: &T, shift: usize, ignore_default: bool) -> String {
    if val.is_default() && !ignore_default {
        format!("{}", val.field_to(shift))
    } else {
        format!("{} = {}", val.field_to(shift), val.value_to(shift))
    }
}

fn field_to_string_impl(val: &FieldType, shift: usize) -> String {
    format!("{}{}: {}", SHIFT.repeat(shift), 
        val.name(), 
        match val.value() {
            Element::Boolean(v) => { field_values_to_string(v, shift, false) },
            Element::String(v) => { field_values_to_string(v, shift, false) },
            Element::Integer(v) => { field_values_to_string(v, shift, false) },
            Element::Floating(v) => { field_values_to_string(v, shift, false) },
            Element::Object(v) => { field_values_to_string(v, shift, false) },
            _ => "".to_string(),
        }
    )
}

fn values_to_string(val: &FieldType, shift: usize) -> String {
    format!("{}{}: {}", SHIFT.repeat(shift), 
        val.name(), 
        match val.value() {
            Element::Boolean(v) => { cast(v).value_to(shift) },
            Element::String(v) => { cast(v).value_to(shift) },
            Element::Integer(v) => { cast(v).value_to(shift) },
            Element::Floating(v) => { cast(v).value_to(shift) },
            Element::Object(v) => { cast(v).value_to(shift) },
            _ => "".to_string(),            
        }
    )
}

pub fn field_to_string(val: &FieldType) -> String {
    field_to_string_impl(val, 0)
}