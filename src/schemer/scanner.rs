#![allow(unused)]

pub struct Scanner<'a> {
    current: &'a str,
    top: char,
    position: (usize, usize),
}

fn get_top(value: &str) -> char {
    value.chars().next().unwrap_or('\0')
}

impl<'a> Scanner<'a> {

    pub fn new (value: &str) -> Scanner {
        Scanner {
            current: value,
            top: get_top(value),
            position: (1, 1), 
        }
    }

    pub fn get(&self) -> &'a str {
        self.current
    }

    pub fn top(&self) -> char {
        self.top
    }

    pub fn position(&self) -> (usize, usize) {
        (self.position.0, self.position.1)
    }

    pub fn jump(&mut self, count: usize) {
        for i in 0..count {
            self.advance();
        }
    }
    pub fn advance_while(&mut self, predic: fn(char)->bool) -> usize {
        let mut count: usize = 0;
        while !self.eol() && predic(self.top()) {
            count += self.advance();
        }
        return count;
    }
    pub fn advance(&mut self) -> usize {
        if self.current.len() > 0 {
            let c: char = self.top;
            let shift = c.len_utf8();
            self.current = &self.current[shift..];
            self.top = get_top(self.current);
            self.position = match c {
                '\n' => (self.position.0 + 1, 1),
                  _  => (self.position.0, self.position.1 + 1),
            };
            return shift;
        } 
        return 0;
    }

    pub fn eol(&self) -> bool {
        return self.current.len() == 0;
    }

    pub fn backup(&self) -> Scanner<'a> {
        return Scanner {
            current: self.current,
            top: self.top,
            position: self.position
        }
    }

    pub fn restore<'b>(&mut self, other: &'b Scanner<'a> ) {
        self.current = other.current;
        self.top = other.top;
        self.position = other.position;
    }
}
