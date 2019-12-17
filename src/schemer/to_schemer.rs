#![allow(unused)]

use super::objects::*;
use super::object_base::*;
use super::helpers::*;

static SHIFT: &'static str = "  ";

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

pub trait ToSchemer {
    fn to_schemer_string(&self) -> String;
}

pub fn to_schemer_string<T: ToSchemer>(obj: &T) -> String {
    obj.to_schemer_string()
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
                field_to_string(v, shift + 1)
            }).collect();
        format!("object{} {{\n{}\n{}}}", 
            if self.is_array() { "[]" } else { "" },
            fields.join(",\n"),
            SHIFT.repeat(0)
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
                format!("[\n{}\n{}]", values.join(",\n"), SHIFT.repeat(shift))
            },
            PossibleArray::Value(val) => {
                let field_info: Vec<String> = self.fields()
                    .iter()
                    .map(|(k, v)| {
                        values_to_string(v, shift + 1)
                    }
                ).collect();
                format!("{}{{\n{}\n{}}}", SHIFT.repeat(shift), &field_info.join(",\n"), SHIFT.repeat(shift))
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

fn field_to_string(val: &FieldType, shift: usize) -> String {
    format!("{}{}: {}", SHIFT.repeat(shift), 
        val.name(), 
        match val.value() {
            Element::Boolean(v) => { field_values_to_string(v, 0, false) },
            Element::String(v) => { field_values_to_string(v, 0, false) },
            Element::Integer(v) => { field_values_to_string(v, 0, false) },
            Element::Floating(v) => { field_values_to_string(v, 0, false) },
            Element::Object(v) => { field_values_to_string(v, 0, false) },
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

impl ToSchemer for FieldType {
    fn to_schemer_string(&self) -> String {
        field_to_string(self, 0)
    }
}

pub fn fild_to_schemer_string(val: &FieldType) -> String {
    field_to_string(val, 0)
}