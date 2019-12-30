
use super::objects::*;
use super::object_base::*;
use super::helpers::*;
use super::formatting::*;

mod utils {
    pub fn is_ident_string(val: &str) -> bool {
        val.chars().find(|c| {
            !(c.is_ascii_alphabetic() || c.is_digit(10) || *c == '_') 
        }).is_none()
    }
    pub fn quote(val: &str) -> String {
        return if !is_ident_string(val) { format!("\"{}\"", val) } else { val.to_string() }
    }
}

trait ToSchemerString {
    fn field_to(&self, format: &Formatting, shift: usize) -> String;
    fn value_to(&self, format: &Formatting, shift: usize) -> String;
}

fn cast<T: ToSchemerString>(val: &T) -> &T {
    val
}

impl ToSchemerString for BooleanType {
    fn field_to(&self, _: &Formatting, _: usize) -> String {
        format!("boolean{}", 
            if self.is_array() { "[]" } else { "" }
        )
    }
    fn value_to(&self, format: &Formatting, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                format!("[{}]", format.format_t_array(arr, shift + 1))
            },
            PossibleArray::Value(val) => {
                format.format_value(val)
            },
        }
    }
}

impl ToSchemerString for StringType {
    fn field_to(&self, format: &Formatting, shift: usize) -> String {
        let opt_enum = self.enum_values();
        let enum_string = match &opt_enum {
            Some(values) => {
                format!(" enum {{{}}}", format.format_t_array(values.values(), shift))
            },
            None => String::new(),
        };
        format!("string{}{}", 
            if self.is_array() { "[]" } else { "" },
            enum_string
        )
    }
    fn value_to(&self, format: &Formatting, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                format!("[{}]", format.format_t_array(&arr, shift + 1))
            },
            PossibleArray::Value(val) => {
                format.format_value(val)
            },
        }
    }
}

impl<T> ToSchemerString for NumberType<T> where T: Numeric + Clone + format::ValueToString {
    fn field_to(&self, format: &Formatting, shift: usize) -> String {
        let ival = self.interval();
        let interval = if ival.has_minmax() { 
            format!(" {}..{}", 
                if ival.has_min() { format.format_value(&ival.min(T::zero())) } else { String::new() }, 
                if ival.has_min() { format.format_value(&ival.max(T::zero())) } else { String::new() } )
        } else {
            String::new()
        };
        let opt_enum = self.enum_values();
        let enum_string = match &opt_enum {
            Some(values) => {
                format!(" enum {{{}}}", 
                    format.format_t_array(values.values(), shift))
            },
            None => String::new(),
        };
        format!("{}{}{}{}", T::name(), 
            if self.is_array() { "[]" } else { "" }, 
            interval, 
            enum_string
        )
    }

    fn value_to(&self, format: &Formatting, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                format!("[{}]", format.format_t_array(arr, shift + 1))
            },
            PossibleArray::Value(val) => {
                val.to_string()
            },
        }
    }
}

impl ToSchemerString for ObjectType {
    fn field_to(&self, format: &Formatting, shift: usize) -> String {

        let fields = format.format_array(
            &self.fields().iter().map(|(_, f)| {
                field_to_string_impl(f, format, shift + 1)
            }).collect::<Vec<String>>(), 
        shift);

        format!("object{} {{{}}}", 
            if self.is_array() { "[]" } else { "" },
            fields,
        )
    }
    fn value_to(&self, format: &Formatting, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                let values = (**arr).iter().map(|x| {
                    match &**x {
                        Some(field) => cast(field).value_to(format, shift + 1),
                        None => String::new(),
                    }
                }).collect::<Vec<String>>();
                format!("[{}]", format.format_array(&values, shift))
            },
            PossibleArray::Value(val) => {
                let fields = match &**val {
                    Some(unboxed) => {
                        unboxed.fields()
                    },
                    None => self.fields(),
                };
                let str_value = fields.iter().map(|(_, v)| {
                    values_to_string(v, format, shift + 1)
                }).collect::<Vec<String>>();
                format!("{{{}}}", format.format_array(&str_value, shift))
            },
        }
    }
}

impl ToSchemerString for AnyType {
    fn field_to(&self, _: &Formatting, _: usize) -> String {
        "any".to_string()
    }
    fn value_to(&self, format: &Formatting, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                let values = (**arr).iter().map(|x| {
                    match &**x {
                        Some(field) => cast(field).value_to(format, shift + 1),
                        None => "null".to_string(),
                    }
                }).collect::<Vec<String>>();
                format!("[{}]", format.format_array(&values, shift))
            },
            PossibleArray::Value(val) => {
                match &**val {
                    Some(unboxed) => {
                        cast(unboxed).value_to(format, shift)
                    },
                    None => "null".to_string(),
                }
            },
        }
    }
}

impl ToSchemerString for Element {
    fn field_to(&self, format: &Formatting, shift: usize) -> String {
        match &self {
            Element::Boolean(v) => { cast(v).field_to(format, shift) },
            Element::String(v) => { cast(v).field_to(format, shift) },
            Element::Integer(v) => { cast(v).field_to(format, shift) },
            Element::Floating(v) => { cast(v).field_to(format, shift) },
            Element::Object(v) => { cast(v).field_to(format, shift) },
            Element::Any(v) => { cast(v).field_to(format, shift) },
            Element::None => "".to_string(),
        }
    }
    fn value_to(&self, format: &Formatting, shift: usize) -> String {
        match &self {
            Element::Boolean(v) => { cast(v).value_to(format, shift) },
            Element::String(v) => { cast(v).value_to(format, shift) },
            Element::Integer(v) => { cast(v).value_to(format, shift) },
            Element::Floating(v) => { cast(v).value_to(format, shift) },
            Element::Object(v) => { cast(v).value_to(format, shift) },
            Element::Any(v) => { cast(v).value_to(format, shift) },
            Element::None => "null".to_string(),
        }
    }
}

fn field_values_to_string<T: ObjectBase + ToSchemerString>(val: &T, format: &Formatting, shift: usize, ignore_default: bool) -> String {
    if val.is_default() && !ignore_default {
        format!("{}", val.field_to(format, shift))
    } else {
        format!("{} = {}", val.field_to(format, shift), val.value_to(format, shift))
    }
}

fn options_to_string(opts: &Options, format: &Formatting, shift: usize) -> String {
    if opts.empty() {
        String::new()
    } else {
        let vals = opts.all().iter().map(|(k, v)|{
            format!("{}: {}", k, cast(v).value_to(format, shift + 1))
        }).collect::<Vec<String>>();
        format!("({})", format.format_array(&vals, shift + 1))
    }
}

fn field_to_string_impl(val: &FieldType, format: &Formatting, shift: usize) -> String {
    format!("{}{}: {}", 
        &utils::quote(val.name()), 
        &options_to_string(&val.options(), format, shift),
        match val.value() {
            Element::Boolean(v) => { field_values_to_string(v, format, shift, false) },
            Element::String(v) => { field_values_to_string(v, format, shift, false) },
            Element::Integer(v) => { field_values_to_string(v, format, shift, false) },
            Element::Floating(v) => { field_values_to_string(v, format, shift, false) },
            Element::Object(v) => { field_values_to_string(v, format, shift, false) },
            Element::Any(v) => { field_values_to_string(v, format, shift, false) },
            Element::None => "".to_string(),
        }
    )
}

fn values_to_string(val: &FieldType, format: &Formatting, shift: usize) -> String {
    format!("{}: {}", 
        utils::quote(val.name()),
        cast(val.value()).value_to(format, shift)
    )
}

pub fn field_to_string(val: &FieldType, shift: usize) -> String {
    let format = Formatting::new(shift);
    field_to_string_impl(val, &format, 0)
}
