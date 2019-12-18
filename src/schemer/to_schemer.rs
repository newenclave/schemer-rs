#![allow(unused)]

use super::objects::*;
use super::object_base::*;
use super::helpers::*;


mod utils {
    static SHIFT: &'static str = "  ";
    pub fn string_join<T: std::string::ToString>(vals: &Vec<T>, sep: &str) -> String {
        vals.iter().map(|v|{
            v.to_string()
        }).collect::<Vec<String>>().join(sep)
    }
    pub fn sh(shift: usize) -> String {
        SHIFT.repeat(shift)
    }
    pub fn is_ident_string(val: &str) -> bool {
        !val.chars().find(|c| {
            !(c.is_ascii_alphabetic() || c.is_digit(10) || *c == '_') 
        }).is_none()
    }
    pub fn quote(val: &str) -> String {
        return if is_ident_string(val) { format!("\"{}\"", val) } else { val.to_string() }
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
        let fields = self.fields()
            .iter()
            .map(|(_, v)| {
                field_to_string_impl(v, shift + 1)
            }).collect::<Vec<String>>().join(",\n");
        format!("object{} {{\n{}\n{}}}", 
            if self.is_array() { "[]" } else { "" },
            fields,
            utils::sh(shift)
        )
    }
    fn value_to(&self, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
            let values = (**arr)
                .iter().map(|x| {
                    match &**x {
                        Some(field) => cast(field).value_to(shift + 1),
                        None => String::new(),
                    }
                }).collect::<Vec<String>>().join(", ");
                format!("[\n{}{}\n{}]", utils::sh(shift + 1), values, utils::sh(shift))
            },
            PossibleArray::Value(val) => {
                let field_info = self.fields()
                    .iter().map(|(k, v)| {
                        values_to_string(v, shift + 1)
                    }
                ).collect::<Vec<String>>().join(",\n");
                format!("{{\n{}\n{}}}", &field_info, utils::sh(shift))
            },
        }
    }
}

impl ToSchemerString for Element {
    fn field_to(&self, shift: usize) -> String {
        match &self {
            Element::Boolean(v) => { cast(v).field_to(shift) },
            Element::String(v) => { cast(v).field_to(shift) },
            Element::Integer(v) => { cast(v).field_to(shift) },
            Element::Floating(v) => { cast(v).field_to(shift) },
            Element::Object(v) => { cast(v).field_to(shift) },
            _ => "".to_string(),
        }
    }
    fn value_to(&self, shift: usize) -> String {
        match &self {
            Element::Boolean(v) => { cast(v).value_to(shift) },
            Element::String(v) => { cast(v).value_to(shift) },
            Element::Integer(v) => { cast(v).value_to(shift) },
            Element::Floating(v) => { cast(v).value_to(shift) },
            Element::Object(v) => { cast(v).value_to(shift) },
            _ => "".to_string(),
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

fn options_to_string(opts: &Options) -> String {
    if opts.empty() {
        String::new()
    } else {
        format!("({})", opts.all()
            .iter().map(|(k, v)|{
                format!("{}: {}", k, cast(v).value_to(0))
            }).collect::<Vec<String>>().join(", ")
        )
    }
}


fn field_to_string_impl(val: &FieldType, shift: usize) -> String {
    format!("{}{}{}: {}", utils::sh(shift), 
        &utils::quote(val.name()), 
        &options_to_string(&val.options()),
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
    format!("{}{}: {}", utils::sh(shift), 
        utils::quote(val.name()),
        cast(val.value()).value_to(shift)
    )
}

pub fn field_to_string(val: &FieldType) -> String {
    field_to_string_impl(val, 0)
}
