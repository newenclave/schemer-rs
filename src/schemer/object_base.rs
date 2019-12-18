
use super::objects::*;
use super::helpers::*;

pub trait ObjectBase {
    fn create() -> Self;
    fn is_array(&self) -> bool;
    fn is_default(&self) -> bool;
    fn make_array(&mut self);
}

impl ObjectBase for StringType {
    fn create() -> Self {
        StringType::new()
    }
    fn is_array(&self) -> bool {
        match self.value() {
            PossibleArray::Array(_) => true,
            _ => false,
        }
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => v.len() == 0,
            PossibleArray::Array(v) => v.len() == 0,
        }        
    }    
    fn make_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()))
    }
}

impl ObjectBase for BooleanType {
    fn create() -> Self {
        BooleanType::new()
    }
    fn is_array(&self) -> bool {
        self.value().is_array()
    }
    fn make_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()));
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => !*v,
            PossibleArray::Array(v) => v.len() == 0,
        }
    }
}

impl ObjectBase for ObjectType {
    fn create() -> Self {
        ObjectType::new()
    }
    fn is_array(&self) -> bool {
        self.value().is_array()
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => !v.is_none(),
            PossibleArray::Array(v) => v.len() == 0,
        }
    }    
    fn make_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()));
    }
}

impl<T> ObjectBase for NumberType<T> where T: Numeric, T: Clone { 
    fn create() -> NumberType<T> {
        NumberType::new()
    }
    fn is_array(&self) -> bool {
        self.value().is_array()
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => *v == T::zero(),
            PossibleArray::Array(v) => v.len() == 0,
        }
    }
    fn make_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()));
    }
}

impl ObjectBase for AnyType {
    fn create() -> Self {
        AnyType::new()
    }
    fn is_array(&self) -> bool {
        self.value().is_array()
    }
    fn is_default(&self) -> bool {
        match self.value() {
            PossibleArray::Value(v) => !v.is_none(),
            PossibleArray::Array(v) => v.len() == 0,
        }
    }
    fn make_array(&mut self) {
        self.set_value(PossibleArray::Array(Vec::new()));
    }
}
