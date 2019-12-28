
use super::objects::*;
use super::object_base::*;
use super::helpers::*;
use super::formatting::{element_format};


mod to_json_schema {
    use super::*;

    pub trait SchemaToValues {
        fn value_to_schema(&self, opts: &Options) -> Element;
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
    impl SchField for &Vec<&str> {
        fn value(self) -> Element {
            let mut arr = StringType::new_array();
            for v in self {
                arr.add_value(v);
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

    impl SchField for bool {
        fn value(self) -> Element {
            Element::Boolean(BooleanType::from(self))
        }
    }

    fn field<T: SchField>(name: &str, val: T) -> FieldType {
        FieldType::new(name.to_string(), val.value(), Options::new())
    }

    fn value<T: SchField>(val: T) -> Element {
        val.value()
    }

    fn set_common_schema_options(obj: &mut ObjectType, opts: &Options) {
        if opts.has_bool("readonly") {
            obj.add_field(field("readonly", true))
        }
    }

    /// TODO: alot of copy-paste 
    impl SchemaToValues for Element {
        fn value_to_schema(&self, opts: &Options) -> Element {
            match self {
                Element::Boolean(v) => { to_json_schema_impl(v, opts) },
                Element::String(v) => { to_json_schema_impl(v, opts) },
                Element::Integer(v) => { to_json_schema_impl(v, opts) },
                Element::Floating(v) => { to_json_schema_impl(v, opts) },
                Element::Object(v) => { to_json_schema_impl(v, opts) },
                Element::Any(v) => { to_json_schema_impl(v, opts) },
                //Element::None => "".to_string(),
                _ => Element::None,
            }
        }
    }

    impl SchemaToValues for BooleanType {
        fn value_to_schema(&self, opts: &Options) -> Element {
            let mut obj = ObjectType::new();
            obj.add_field(field("type", "boolean"));
            if self.is_array() {
                let mut arr = ObjectType::new();
                set_common_schema_options(&mut arr, opts);
                arr.add_field(field("type", "array"));
                arr.add_field(field("items", obj));
                return value(arr);
            } else {
                set_common_schema_options(&mut obj, opts);
                return value(obj);
            }
        }
    }

    impl SchemaToValues for IntegerType {
        fn value_to_schema(&self, opts: &Options) -> Element {
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
                set_common_schema_options(&mut arr, opts);
                arr.add_field(field("type", "array"));
                arr.add_field(field("items", obj));
                return value(arr);
            } else {
                set_common_schema_options(&mut obj, opts);
                return value(obj);
            }
        }
    }

    impl SchemaToValues for FloatingType {
        fn value_to_schema(&self, opts: &Options) -> Element {
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
                set_common_schema_options(&mut arr, opts);
                arr.add_field(field("type", "array"));
                arr.add_field(field("items", obj));
                return value(arr);
            } else {
                set_common_schema_options(&mut obj, opts);
                return value(obj);
            }
        }
    }

    impl SchemaToValues for StringType {
        fn value_to_schema(&self, opts: &Options) -> Element {
            let mut obj = ObjectType::new();
            obj.add_field(field("type", "string"));

            match self.enum_values() {
                Some(vals) => obj.add_field(field("enum", vals.values())),
                None => {},
            }

            if self.is_array() {
                let mut arr = ObjectType::new();
                set_common_schema_options(&mut obj, opts);
                arr.add_field(field("type", "array"));
                arr.add_field(field("items", obj));
                return value(arr);
            } else {
                set_common_schema_options(&mut obj, opts);
                return value(obj);
            }
        }
    }

    impl SchemaToValues for ObjectType {
        fn value_to_schema(&self, opts: &Options) -> Element {
            let mut obj = ObjectType::new();
            obj.add_field(field("type", "object"));
            
            let mut props = ObjectType::new();
            for (k, v) in self.fields() {
                props.add_field(field(k, to_json_schema_impl(v.value(), v.options())))
            }
            let required = self.fields().iter()
                .filter(|(_, v)| v.options().has_bool("required") )
                .map(|(k, _)| k.to_string() )
                .collect::<Vec<String>>();
            if required.len() > 0 {
                obj.add_field(field("required", value(&required)));
            }

            obj.add_field(field("properties", props));
            if self.is_array() {
                let mut arr = ObjectType::new();
                set_common_schema_options(&mut arr, opts);
                arr.add_field(field("type", "array"));
                arr.add_field(field("items", obj));
                return value(arr);
            } else {
                set_common_schema_options(&mut obj, opts);
                return value(obj);
            }
        }
    }

    impl SchemaToValues for AnyType {
        fn value_to_schema(&self, opts: &Options) -> Element {
            match self.value() {
                PossibleArray::Value(opt_val) => {
                    match &**opt_val {
                        Some(val) => to_json_schema_impl(val, opts),
                        None => {
                            let mut obj = ObjectType::new();
                            //obj.add_field(field("type", &vec!("object", "null")));
                            set_common_schema_options(&mut obj, opts);
                            value(obj)
                        },
                    }
                }
                PossibleArray::Array(_) => {
                    let mut obj = ObjectType::new();
                    set_common_schema_options(&mut obj, opts);
                    obj.add_field(field("type", "array"));
                    value(obj)
                },
            }
        }
    }

    pub fn to_json_schema_impl<T: to_json_schema::SchemaToValues>(val: &T, opts: &Options) -> Element {
        val.value_to_schema(opts)
    }
}

pub fn to_json_values(val: &FieldType) -> String {
    element_format(val.value(), 2)
}

pub fn to_json_schema(val: &FieldType) -> String {
    use to_json_schema::to_json_schema_impl as call_impl;
    let schema_obj = match val.value() {
        Element::Boolean(v) => { call_impl(v, val.options()) },
        Element::String(v) => { call_impl(v, val.options()) },
        Element::Integer(v) => { call_impl(v, val.options()) },
        Element::Floating(v) => { call_impl(v, val.options()) },
        Element::Object(v) => { call_impl(v, val.options()) },
        Element::Any(v) => { call_impl(v, val.options()) },
        //Element::None => "".to_string(),
        _ => Element::None,
    };
    element_format(&schema_obj, 2)
}
