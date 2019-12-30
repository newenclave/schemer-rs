use std::collections::HashMap;
use super::objects::{Element};

#[derive(Clone)]
pub struct Environment {
    aliases: HashMap<String, Element> 
}

impl Environment {
    #![allow(unused)]
    pub fn new() -> Environment {
        Environment {
            aliases: HashMap::new()
        }
    }
    pub fn set_alias(&mut self, k: &str, val: Element) {
        self.aliases.insert(String::from(k), val);
    }
    pub fn has_alias(&self, k: &str) -> bool {
        self.aliases.get(k).is_some()
    }
}
