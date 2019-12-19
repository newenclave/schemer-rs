
use std::collections::HashMap;
use super::helpers::*;

#[derive(Clone)]
pub struct StringType {
    value: PossibleArray<String>,
    enum_values: Option<Enum<String>>,
}


impl StringType {
    pub fn new() -> StringType {
        StringType {
            value: PossibleArray::Value(String::new()),
            enum_values: None,
        }
    }
    pub fn new_array() -> StringType {
        StringType {
            value: PossibleArray::Array(Vec::new()),
            enum_values: None,
        }
    }
    pub fn from(val: &str) -> StringType {
        StringType {
            value: PossibleArray::Value(val.to_string()),
            enum_values: None,
        }
    }
    
    pub fn check_enum(&self, val: &String) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
    }

    pub fn value(&self) -> &PossibleArray<String> {
        &self.value
    }

    pub fn set_value(&mut self, val: PossibleArray<String>) {
        self.value = val;
    }

    pub fn add_value(&mut self, value: &str) {
        self.value.add_value(String::from(value));
    }
    
    pub fn enum_values(&self) -> &Option<Enum<String>> {
        &self.enum_values
    }
    
    pub fn add_enum_value(&mut self, val: &str) -> bool {
        match &mut self.enum_values {
            Some(values) => values.try_add(val.to_string()),
            None => { 
                self.add_value(val);
                self.enum_values = Some(Enum::create_with(val.to_string()));
                true
            },
        }
    }
}

#[derive(Clone)]
pub struct NumberType<T: Clone> {
    value: PossibleArray<T>,
    min_max: Interval<T>,
    enum_values: Option<Enum<T>>,
}

impl <T> NumberType<T> where T: Numeric {
    pub fn new() -> NumberType<T> {
        NumberType {
            value: PossibleArray::Value(T::zero()),
            min_max: Interval::none(),
            enum_values: None,
        }
    }
    pub fn new_array() -> NumberType<T> {
        NumberType {
            value: PossibleArray::Array(Vec::new()),
            min_max: Interval::none(),
            enum_values: None,
        }
    }
    pub fn add_value(&mut self, value: T) {
        self.value.add_value(value);
    }

    pub fn check_enum(&self, val: T) -> bool {
        match &self.enum_values {
            Some(vals) => vals.check(&val),
            None => true,
        }
    }

    pub fn value(&self) -> &PossibleArray<T> {
        &self.value
    }

    pub fn set_value(&mut self, val: PossibleArray<T>) {
        self.value = val;
    }

    pub fn interval(&self) -> &Interval<T> {
        &self.min_max
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

    pub fn enum_values(&self) -> &Option<Enum<T>> {
        &self.enum_values
    }

    pub fn add_enum_value(&mut self, val: T) -> bool {
        match &mut self.enum_values {
            Some(values) => values.try_add(val),
            None => { 
                self.add_value(val.clone());
                self.enum_values = Some(Enum::create_with(val));
                true
            },
        }
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
    pub fn from(val: bool) -> BooleanType {
        BooleanType{
            value: PossibleArray::Value(val),
        }
    }
    pub fn add_value(&mut self, value: bool) {
        self.value.add_value(value);
    }

    pub fn value(&self) -> &PossibleArray<bool> {
        &self.value
    }

    pub fn set_value(&mut self, val: PossibleArray<bool>) {
        self.value = val;
    }
}

#[derive(Clone)]
pub struct ObjectType {
    value: PossibleArray<Box<Option<ObjectType>>>,
    opts: Options,
    fields: HashMap<String, FieldType>,
}

impl ObjectType {
    pub fn new() -> ObjectType {
        ObjectType {
            value: PossibleArray::Value(Box::new(None)),
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

    pub fn fields(&self) -> &HashMap<String, FieldType> {
        &self.fields
    }
    
    pub fn value(&self) -> &PossibleArray<Box<Option<ObjectType>>> {
        &self.value
    }
    pub fn set_value(&mut self, val: PossibleArray<Box<Option<ObjectType>>>) {
        self.value = val;
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
}

#[derive(Clone)]
pub struct AnyType {
    value: PossibleArray<Box<Option<Element>>>,
    opts: Options,
}

impl AnyType {
    pub fn new() -> AnyType {
        AnyType {
            value: PossibleArray::Value(Box::new(None)),
            opts: Options::new(),
        }
    }
    pub fn new_array() -> AnyType {
        AnyType {
            value: PossibleArray::new_array(),
            opts: Options::new(),
        }
    }
    pub fn add_value(&mut self, value: Element) {
        self.value.add_value(Box::new(Some(value)))
    }
    pub fn value(&self) -> &PossibleArray<Box<Option<Element>>> {
        &self.value
    }
    pub fn set_value(&mut self, val: PossibleArray<Box<Option<Element>>>) {
        self.value = val;
    }
}

#[derive(Clone)]
pub enum Element {
    None,
    String(StringType),
    Integer(IntegerType),
    Floating(FloatingType),
    Boolean(BooleanType),
    Object(ObjectType),
    Any(AnyType),
}

#[derive(Clone)]
pub struct Options {
    values: HashMap<String, Element>
}
impl Options {
    pub fn new() -> Options {
        Options {
            values: HashMap::new(),
        }
    }
    pub fn empty(&self) -> bool {
        self.values.len() == 0
    }
    pub fn add(&mut self, key: &str, value: Element) {
        self.values.insert(key.to_string(), value);
    }
    pub fn all(&self) -> &HashMap<String, Element> {
        &self.values
    }
}

#[derive(Clone)]
pub struct FieldType {
    value: Element,
    name: String,
    opts: Options,
}

impl FieldType {
    pub fn new(name: String, value: Element, opts: Options) -> FieldType {
        FieldType{
            value: value,
            name: String::from(name),
            opts: opts,
        }
    }
    pub fn value<'a>(&'a self) -> &'a Element {
        return &self.value
    }
    pub fn name(&self) -> &str {
        return &self.name
    }
    pub fn options(&self) -> &Options {
        &self.opts
    }
}
