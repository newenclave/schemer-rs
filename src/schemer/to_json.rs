#![allow(unused)]

use super::objects::*;
use super::object_base::*;
use super::helpers::*;

// TODO: remove copy-paste
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

    pub fn sh_str(shift: usize, value: &str) -> String {
        format!("{}{}", sh(shift), value)
    }
}

// TODO: looks like copy-paste from to_schemer. 
//  Need to provide some trait with plain string but with no format
trait ValuesToJson {
    fn value_to_json(&self, shift: usize) -> String;
}

impl ValuesToJson for BooleanType {
    fn value_to_json(&self, shift: usize) -> String {
        match self.value() {
            PossibleArray::Value(val) => { val.to_string() },
            PossibleArray::Array(val) => {
                utils::sh_str(shift, &format!("[{}]",
                    &utils::string_join(&val.iter().map(|v| v.to_string()).collect::<Vec<String>>(), ", ")
                ))
            },
        }
    }
}

impl<T> ValuesToJson for NumberType<T> where T: Numeric {
    fn value_to_json(&self, shift: usize) -> String {
        match self.value() {
            PossibleArray::Value(val) => { val.to_string() },
            PossibleArray::Array(val) => {
                format!("[{}]",
                    &utils::string_join(&val.iter().map(|v| v.to_string()).collect::<Vec<String>>(), ", ")
                )
            },
        }
    }
}

impl ValuesToJson for StringType {
    fn value_to_json(&self, shift: usize) -> String {
        match self.value() {
            PossibleArray::Value(val) => { format!("\"{}\"", val.to_string()) },
            PossibleArray::Array(val) => {
                format!("[{}]",
                    &utils::string_join(&val.iter()
                        .map(|v| {
                            format!("\"{}\"", v.to_string())
                        }).collect::<Vec<String>>(), ", ")
                )
            },
        }
    }
}

impl ValuesToJson for ObjectType {
    fn value_to_json(&self, shift: usize) -> String {
        match self.value() {
            PossibleArray::Value(val) => {
                let str_value = match &**val {
                    Some(unboxed) => {
                        unboxed.fields()
                    },
                    None => self.fields(),
                }.iter().map(|(k, v)| {
                    to_json_values_impl(k, v.value(), shift + 1)
                }).collect::<Vec<String>>().join(",\n");
                if str_value.len() == 0 {
                    "{}".to_string()
                } else {
                    format!("{{\n{}\n{}}}", &str_value, utils::sh(shift))
                }
            },
            PossibleArray::Array(arr) => {
                let values = (**arr)
                .iter().map(|x| {
                    match &**x {
                        Some(field) => field.value_to_json(shift + 1),
                        None => String::new(),
                    }
                }).collect::<Vec<String>>().join(", ");
                if values.len() == 0 {
                    "[]".to_string()
                } else {
                    format!("[\n{}{}\n{}]", utils::sh(shift + 1), values, utils::sh(shift))
                }
            },
        }
    }
}

impl ValuesToJson for AnyType {
    fn value_to_json(&self, shift: usize) -> String {
        match self.value() {
            PossibleArray::Array(arr) => {
                let values = (**arr)
                .iter().map(|x| {
                    match &**x {
                        Some(field) => element_to_json_values_impl(field, shift + 1),
                        None => "null".to_string(),
                    }
                }).collect::<Vec<String>>().join(", ");
                if values.len() == 0 {
                    "[]".to_string()
                } else {
                    format!("[\n{}{}\n{}]", utils::sh(shift + 1), values, utils::sh(shift))
                }
            },
            PossibleArray::Value(val) => {
                match &**val {
                    Some(unboxed) => {
                        element_to_json_values_impl(unboxed, shift)
                    },
                    None => "null".to_string(),
                }
            },
        }
    }
}

trait SchemaToValues {
    fn value_to_schema(&self) -> Element;
}

trait SchField {
    fn value(self) -> Element;
}

impl SchField for &str {
    fn value(self) -> Element {
        Element::String(StringType::from(self))
    }
}

impl SchField for Element {
    fn value(self) -> Element {
        self
    }
}

impl SchField for &Vec<String> {
    fn value(self) -> Element {
        let mut arr = StringType::new_array();
        for v in self {
            arr.add_value(&v);
        };
        Element::String(arr)
    }
}

impl SchField for &Vec<i64> {
    fn value(self) -> Element {
        let mut arr = IntegerType::new_array();
        for v in self {
            arr.add_value(*v);
        };
        Element::Integer(arr)
    }
}
impl SchField for &Vec<f64> {
    fn value(self) -> Element {
        let mut arr = FloatingType::new_array();
        for v in self {
            arr.add_value(*v);
        };
        Element::Floating(arr)
    }
}

impl SchField for ObjectType {
    fn value(self) -> Element {
        Element::Object(self)
    }
}

impl SchField for f64 {
    fn value(self) -> Element {
        Element::Floating(FloatingType::from(self))
    }
}

impl SchField for i64 {
    fn value(self) -> Element {
        Element::Integer(IntegerType::from(self))
    }
}

fn field<T: SchField>(name: &str, val: T) -> FieldType {
    FieldType::new(name.to_string(), val.value(), Options::new())
}

fn value<T: SchField>(val: T) -> Element {
    val.value()
}

/// TODO: alot of copy-paste 
impl SchemaToValues for Element {
    fn value_to_schema(&self) -> Element {
        match self {
            Element::Boolean(v) => { to_json_schema_impl(v) },
            Element::String(v) => { to_json_schema_impl(v) },
            Element::Integer(v) => { to_json_schema_impl(v) },
            Element::Floating(v) => { to_json_schema_impl(v) },
            Element::Object(v) => { to_json_schema_impl(v) },
            // Element::Any(v) => { call_value_to_json(v, shift) },
            //Element::None => "".to_string(),
            _ => Element::None,
        }
    }
}

impl SchemaToValues for BooleanType {
    fn value_to_schema(&self) -> Element {
        let mut obj = ObjectType::new();
        obj.add_field(field("type", "boolean"));
        if self.is_array() {
            let mut arr = ObjectType::new();
            arr.add_field(field("type", "array"));
            arr.add_field(field("items", obj));
            return value(arr);
        } else {
            return value(obj);
        }
    }
}

impl SchemaToValues for IntegerType {
    fn value_to_schema(&self) -> Element {
        let mut obj = ObjectType::new();
        obj.add_field(field("type", "integer"));
        match self.enum_values() {
            Some(vals) => obj.add_field(field("enum", vals.values())),
            None => {},
        }
        if self.interval().has_min() {
            obj.add_field(field("minimum", self.interval().min(0)));            
        }
        if self.interval().has_max() {
            obj.add_field(field("maximum", self.interval().max(0)));            
        }
        if self.is_array() {
            let mut arr = ObjectType::new();
            arr.add_field(field("type", "array"));
            arr.add_field(field("items", obj));
            return value(arr);
        } else {
            return value(obj);
        }
    }
}

impl SchemaToValues for FloatingType {
    fn value_to_schema(&self) -> Element {
        let mut obj = ObjectType::new();
        obj.add_field(field("type", "number"));
        match self.enum_values() {
            Some(vals) => obj.add_field(field("enum", vals.values())),
            None => {},
        }
        if self.interval().has_min() {
            obj.add_field(field("minimum", self.interval().min(0.0)));            
        }
        if self.interval().has_max() {
            obj.add_field(field("maximum", self.interval().max(0.0)));            
        }
        if self.is_array() {
            let mut arr = ObjectType::new();
            arr.add_field(field("type", "array"));
            arr.add_field(field("items", obj));
            return value(arr);
        } else {
            return value(obj);
        }
    }
}

impl SchemaToValues for StringType {
    fn value_to_schema(&self) -> Element {
        let mut obj = ObjectType::new();
        obj.add_field(field("type", "string"));

        match self.enum_values() {
            Some(vals) => obj.add_field(field("enum", vals.values())),
            None => {},
        }

        if self.is_array() {
            let mut arr = ObjectType::new();
            arr.add_field(field("type", "array"));
            arr.add_field(field("items", obj));
            return value(arr);
        } else {
            return value(obj);
        }
    }
}

impl SchemaToValues for ObjectType {
    fn value_to_schema(&self) -> Element {
        let mut obj = ObjectType::new();
        obj.add_field(field("type", "object"));

        let mut props = ObjectType::new();
        for (k, v) in self.fields() {
            props.add_field(field(k, to_json_schema_impl(v.value())))
        }
        obj.add_field(field("properties", props));
        if self.is_array() {
            let mut arr = ObjectType::new();
            arr.add_field(field("type", "array"));
            arr.add_field(field("items", obj));
            return value(arr);
        } else {
            return value(obj);
        }
    }
}

fn call_value_to_json<T: ValuesToJson + ObjectBase>(val: &T, shift: usize) -> String {
    val.value_to_json(shift)
}

fn element_to_json_values_impl(val: &Element, shift: usize) -> String {
    match val {
        Element::Boolean(v) => { call_value_to_json(v, shift) },
        Element::String(v) => { call_value_to_json(v, shift) },
        Element::Integer(v) => { call_value_to_json(v, shift) },
        Element::Floating(v) => { call_value_to_json(v, shift) },
        Element::Object(v) => { call_value_to_json(v, shift) },
        Element::Any(v) => { call_value_to_json(v, shift) },
        Element::None => "".to_string(),
    }
}

fn to_json_values_impl(name: &str, val: &Element, shift: usize) -> String {
    format!("{}\"{}\": {}", utils::sh(shift), 
        name,
        element_to_json_values_impl(val, shift)
    )
}

pub fn to_json_values(val: &FieldType) -> String {
    element_to_json_values_impl(val.value(), 0)
    //to_json_values_impl(val.name(), val.value(), 0)
}

fn to_json_schema_impl<T: SchemaToValues>(val: &T) -> Element {
    val.value_to_schema()
}

pub fn to_json_schema(val: &FieldType) -> String {
    let schema_obj = match val.value() {
        Element::Boolean(v) => { to_json_schema_impl(v) },
        Element::String(v) => { to_json_schema_impl(v) },
        Element::Integer(v) => { to_json_schema_impl(v) },
        Element::Floating(v) => { to_json_schema_impl(v) },
        Element::Object(v) => { to_json_schema_impl(v) },
        // Element::Any(v) => { call_value_to_json(v, shift) },
        //Element::None => "".to_string(),
        _ => Element::None,
    };
    element_to_json_values_impl(&schema_obj, 0)
    //to_json_values_impl(val.name(), &schema_obj, 0)
}
