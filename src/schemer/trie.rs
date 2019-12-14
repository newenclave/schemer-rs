#![allow(unused)]

use std::collections::HashMap;
use super::scanner::Scanner as StrScanner;

pub struct Trie<T> {
    data: Option<T>,
    children: HashMap<char, Trie<T>>
}

impl<T> Trie<T> {

    pub fn new() -> Trie<T> {
        Trie {
            data: None,
            children: HashMap::new()
        }
    }

    pub fn get<'a, 'b>(&'a self, data: &'b mut StrScanner) -> Option<(&T, usize)> {
        let mut root = self;
        let mut last: Option<&T> = None;
        let mut last_shift: usize = 0;
        let mut shift: usize = 0;
        let other = data.backup();

        while !data.eol() {
            let c = data.top();
            let next = root.children.get(&c);
            match next {
                Some(expr) => {
                    root = &expr;
                    if !root.data.is_none() {
                        last = root.value_ref();
                        last_shift = shift + c.len_utf8();
                    }
                },
                None => break,
            }
            shift += data.advance();
        }

        return match last {
            Some(expr) => Some((expr, last_shift)),
            None => { 
                data.restore(&other); 
                None 
            }
        }
    }

    pub fn set(&mut self, key: &str, value: T) -> bool {
        let mut root = self;
        for c in key.chars() {
            root = root.children.entry(c).or_insert(Trie::new());
        }
        root.data = Some(value);
        return true;
    }

    fn value_ref(&self) -> Option<&T> {
        match &self.data {
            Some(expr) => Some(&expr),
            None => None,
        }
    }
}
