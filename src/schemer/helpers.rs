
#[derive(Clone)]
pub struct Interval<T: Clone> {
    min_max: (Option<T>, Option<T>),
}

impl<T> Interval<T> where T: std::cmp::PartialOrd + Copy {
    pub fn none() -> Interval<T> {
        Interval {
            min_max: (None, None)
        }
    }
    pub fn check(&self, val: T) -> bool {
        return match &self.min_max.0 {
            Some(min_val) => *min_val <= val,
            None => true,
        } && match &self.min_max.1 {
            Some(max_val) => val <= *max_val,
            None => true,
        }
    }
    pub fn set_min(&mut self, val: T) {
        self.min_max.0 = Some(val);
    }
    pub fn set_max(&mut self, val: T) {
        self.min_max.1 = Some(val);
    }

    pub fn has_min(&self) -> bool {
        self.min_max.0.is_some()
    }
    pub fn has_max(&self) -> bool {
        self.min_max.1.is_some()
    }
    pub fn has_minmax(&self) -> bool {
        self.has_min() || self.has_max()
    }
    pub fn min(&self, default: T) -> T {
        match &self.min_max.0 {
            Some(val) => *val,
            None => default,
        }
    }
    pub fn max(&self, default: T) -> T {
        match &self.min_max.1 {
            Some(val) => *val,
            None => default,
        }
    }
}

#[derive(Clone)]
pub struct Enum<T: Clone> {
    values: Vec<T>
}

impl<T> Enum<T> where T: std::cmp::PartialEq + Clone {
    pub fn create_with(val: T) -> Enum<T> {
        Enum {
            values: vec!(val),
        }
    }
    pub fn check(&self, val: &T) -> bool {
        self.values.iter().find(|&x| *val == *x).is_some()
    }
    pub fn try_add(&mut self, val: T) -> bool {
        if self.check(&val) {
            false
        } else {
            self.values.push(val);
            true
        }
    }
    pub fn values(&self) -> &Vec<T> {
        &self.values
    }
}

#[derive(Clone)]
pub enum PossibleArray<T: Clone> {
    Value(T),
    Array(Vec<T>),
}

impl<T> PossibleArray<T> where T: Clone {
    pub fn new_array() -> Self {
        PossibleArray::Array(Vec::new())
    }
    pub fn add_value(&mut self, value: T) {
        match self {
            PossibleArray::Value(val) => { *val = value },
            PossibleArray::Array(vec) => { vec.push(value) },
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            PossibleArray::Value(_) => false,
            PossibleArray::Array(_) => true,
        }
    }
    pub fn as_value(&self) -> Option<&T> {
        match &self {
            PossibleArray::Value(val) => Some(&val),
            PossibleArray::Array(_) => None,
        }
    }
    pub fn as_array(&self) -> Option<&Vec<T>> {
        match &self {
            PossibleArray::Value(_) => None,
            PossibleArray::Array(arr) => Some(arr),
        }
    }
}

pub trait Numeric: Copy + 
                Clone +
                PartialOrd + 
                PartialEq + 
                std::string::ToString {
    fn zero() -> Self;
    fn name() -> &'static str;
}

impl Numeric for i64 {
    fn zero() -> Self {
        0 as Self
    }
    fn name() -> &'static str {
        "integer"
    }
}

impl Numeric for f64 {
    fn zero() -> Self {
        0.0 as Self
    }
    fn name() -> &'static str {
        "floating"
    }
}
