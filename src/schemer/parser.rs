
use super::tokens::{TokenInfo, Token, SpecialToken, TypeName};
use super::objects::*;
use super::object_base::*;

struct ParserBackup {
    current: usize,
    next: usize,
}

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    next: usize,
    eof_token: TokenInfo, 
}

mod helpers {
    use super::*;

    fn read_sign(parser: &mut Parser) -> i32 {
        if Token::is_special(SpecialToken::Minus)(parser.current().token()) {
            parser.advance();
            -1_i32
        } else if Token::is_special(SpecialToken::Plus)(parser.current().token()) {
            parser.advance();
            1_i32
        } else {
            1_i32
        }
    }

    pub trait WithInterval {
        fn set_min(&mut self, parser: &mut Parser);
        fn set_max(&mut self, parser: &mut Parser);
    }
    
    impl WithInterval for IntegerType {
        fn set_min(&mut self, parser: &mut Parser) {
            let sign = read_sign(parser) as i64;
            match &parser.current().token() {
                Token::Integer(val) => self.set_min(*val * sign),
                _ => (),
            }
        }
        fn set_max(&mut self, parser: &mut Parser) {
            let sign = read_sign(parser) as i64;
            match &parser.current().token() {
                Token::Integer(val) => self.set_max(*val * sign),
                _ => (),
            }
        }
    }

    impl WithInterval for FloatingType {
        fn set_min(&mut self, parser: &mut Parser) {
            let sign = read_sign(parser) as f64;
            match parser.current().token() {
                Token::Floating(val) => self.set_min(*val * sign),
                Token::Integer(val) => self.set_min(*val as f64 * sign),
                _ => (),
            }
        }
        fn set_max(&mut self, parser: &mut Parser) {
            let sign = read_sign(parser) as f64;
            match parser.current().token() {
                Token::Floating(val) => self.set_max(*val * sign),
                Token::Integer(val) => self.set_max(*val as f64 * sign),
                _ => (),
            }
        }
    }

    pub trait WithEnum {
        fn enum_add_value(&mut self, parser: &mut Parser) -> bool;
    }
    
    impl WithEnum for IntegerType {
        fn enum_add_value(&mut self, parser: &mut Parser) -> bool {
            match &parser.current().token() {
                Token::Integer(val) => self.add_enum_value(*val),
                _ => true,
            }
        }
    }

    impl WithEnum for FloatingType {
        fn enum_add_value(&mut self, parser: &mut Parser) -> bool {
            match &parser.current().token() {
                Token::Integer(val) => self.add_enum_value(*val as f64),
                Token::Floating(val) => self.add_enum_value(*val),
                _ => true,
            }
        }
    }

    impl WithEnum for StringType {
        fn enum_add_value(&mut self, parser: &mut Parser) -> bool {
            match &parser.current().token() {
                Token::String(val) => self.add_enum_value(val),
                _ => true,
            }
        }
    }

    pub trait ValueReadCheck {
        fn token_checker(val: &Token) -> bool;
        fn expected() -> &'static str;
        fn read_value(&mut self, parser: &mut Parser);
    }

    impl ValueReadCheck for StringType {
        fn token_checker(val: &Token) -> bool {
            val.is_string()
        }

        fn expected() -> &'static str {
            "string"
        }

        fn read_value(&mut self, parser: &mut Parser) {
            match parser.current().token() {
                Token::String(val) => {
                    if !self.check_enum(val) {
                        parser.panic_current(&format!("Value '{}' is not valis for enum.", val));
                    }
                    self.add_value(val)
                },
                _ => ()
            }
        }
    }

    impl ValueReadCheck for IntegerType {
        fn token_checker(val: &Token) -> bool {
            match val {
                Token::Integer(_) => true,
                Token::Special(v) => match v {
                    SpecialToken::Plus => true,
                    SpecialToken::Minus => true,
                    _ => false,
                },
                _ => false
            }
        }
        
        fn expected() -> &'static str {
            "integer"
        }

        fn read_value(&mut self, parser: &mut Parser) {
            let sign = read_sign(parser) as i64;
            match parser.current().token() {
                Token::Integer(val) => {
                    let result = *val * sign; 
                    if !self.check_enum(result) {
                        parser.panic_current(&format!("Value {} is invalid for integer enum", result));
                    } else if !self.check_minmax(result) {
                        parser.panic_current(&format!("Value {} is invalid for integer interval", result));
                    } else {
                        self.add_value(result)
                    }
                },
                _ => ()
            }
        }
    }

    impl ValueReadCheck for FloatingType {
        fn token_checker(val: &Token) -> bool {
            match val {
                Token::Integer(_) => true,
                Token::Floating(_) => true,
                Token::Special(v) => match v {
                    SpecialToken::Plus => true,
                    SpecialToken::Minus => true,
                    _ => false,
                },
                _ => false
            }
        }
        fn expected() -> &'static str {
            "floating or integer"
        }
        fn read_value(&mut self, parser: &mut Parser) {
            let sign = read_sign(parser) as f64;
            let val = match parser.current().token() {
                Token::Floating(val) => *val,
                Token::Integer(val) => *val as f64,
                _ => panic!("Should not be here")
            } * sign;

            if !self.check_enum(val) {
                parser.panic_current(&format!("Value {} is invalid for floating enum", val));
            } else if !self.check_minmax(val) {
                parser.panic_current(&format!("Value {} is invalid for floating interval", val));
            } else {
                self.add_value(val)
            }
        }
    }

    impl ValueReadCheck for BooleanType {
        fn token_checker(val: &Token) -> bool {
            val.is_boolean()
        }
        fn expected() -> &'static str {
            "true or false"
        }
        fn read_value(&mut self, parser: &mut Parser) {
            match parser.current().token() {
                Token::Boolean(val) => {
                    self.add_value(*val)
                },
                _ => ()
            }
        }
    }

    fn create_same_object<T: ObjectBase>(val: &T) -> T {
        let mut res = T::create();
        if val.is_array() {
            res.make_array();
        }
        return res;
    }

    impl ValueReadCheck for ObjectType {
        fn token_checker(val: &Token) -> bool {
            Token::is_special(SpecialToken::LBrace)(val)
        }
        fn expected() -> &'static str {
            "{"
        }

        fn read_value(&mut self, parser: &mut Parser) {
            let mut next = ObjectType::new();
            next.set_fields(self.clone_fields());
            while !parser.expect(&Token::is_special(SpecialToken::RBrace)) {
                let (found, field_name) = parser.read_name();
                if !found {
                    if Token::is_special(SpecialToken::RBrace)(parser.next().token()) {
                        break;
                    } else {
                        parser.panic_expect("ident, string or }");
                    }
                }
                let field = self.get_field(&field_name);
                match field {
                    Some(value) => {
                        let opts = Options::new(); //value.options().clone();
                        match value.value() {
                            Element::None => (),
                            Element::String(v) => { 
                                let mut val = create_same_object(v);
                                parser.read_value(&mut val);
                                next.add_field(FieldType::new(field_name, Element::String(val), opts));
                            },
                            Element::Integer(v) => {
                                let mut val = create_same_object(v);
                                parser.read_value(&mut val);
                                next.add_field(FieldType::new(field_name, Element::Integer(val), opts));
                            },
                            Element::Floating(v) => {
                                let mut val = create_same_object(v);
                                parser.read_value(&mut val);
                                next.add_field(FieldType::new(field_name, Element::Floating(val), opts));
                            },
                            Element::Boolean(v) => { 
                                let mut val = create_same_object(v);
                                parser.read_value(&mut val);
                                next.add_field(FieldType::new(field_name, Element::Boolean(val), opts));
                            },
                            Element::Object(v) => { 
                                let mut val = create_same_object(v);
                                val.set_fields(v.clone_fields());
                                parser.read_value(&mut val);
                                next.add_field(FieldType::new(field_name, Element::Object(val), opts));
                            },
                            Element::Any(_) => { 
                                // let mut val = create_same_object(v);
                                // parser.read_value(&mut val);
                                // next.add_field(FieldType::new(field_name, Element::Any(val), opts));
                            },
                        }
                    },
                    None => {
                        if parser.expect(&Token::is_special(SpecialToken::Colon)) ||
                            parser.expect(&Token::is_special(SpecialToken::Equal)) {
                            next.add_field(FieldType::new(field_name, parser.guess_element(), Options::new()));
                        } else {
                            panic!("Object doesn't contain field with name '{}' and it's type cannot be detected", field_name);
                        }
                    },
                }
                parser.expect(&Token::is_special(SpecialToken::Comma));
            }
            self.add_value(next);
        }
    }
}

impl Parser {
    pub fn new(toks: Vec<TokenInfo>) -> Parser {
        let len = toks.len();
        Parser {
            tokens: toks,
            current: 0,
            next: if len == 0 { 0 } else { 1 },
            eof_token: TokenInfo::new(Token::Eof, (len, len)),
        }
    }

    fn backup(&self) -> ParserBackup {
        ParserBackup {
            current: self.current,
            next: self.next,
        }
    }

    fn restore(&mut self, bu: &ParserBackup) {
        self.current = bu.current;
        self.next = bu.next;        
    }

    pub fn advance(&mut self) -> bool {
        self.current = self.next;
        return if self.next == self.tokens.len() {
            false
        } else {
            self.next += 1;
            true
        }
    }

    pub fn eof(&self) -> bool {
        self.current == self.tokens.len()
    }

    pub fn next_eof(&self) -> bool {
        self.next == self.tokens.len()
    }

    pub fn current(&self) -> &TokenInfo {
        return if self.eof() { &self.eof_token } else { &self.tokens[self.current] };
    }

    pub fn next(&self) -> &TokenInfo {
        return if self.next_eof() { &self.eof_token } else { &self.tokens[self.next] };
    }

    fn panic_expect(&self, exp: &str) {
        panic!("unexpected '{}' at {}:{}. Expected '{}'", self.next().to_string(), 
            self.next().position().0, self.next().position().1, exp);
    }

    fn panic_current(&self, exp: &str) {
        panic!("current '{}' at {}:{}. {}", self.current().to_string(), 
            self.current().position().0, self.current().position().1, exp);
    }

    pub fn expect<F: Fn(&Token) -> bool>(&mut self, call: &F) -> bool {
        if call(self.next().token()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_begin<T: ObjectBase>(&mut self, mut result: T) -> T {
        if self.expect(&Token::is_special(SpecialToken::LBracket)) {
            if !self.expect(&Token::is_special(SpecialToken::RBracket)) {
                self.panic_expect("]");
            }
            result.make_array();
        }
        return result;
    }

    fn read_name(&mut self) -> (bool, String) {
        let name = match &self.next().token() {
            Token::Ident(value) => { Some(String::from(value)) },
            Token::String(value) => { Some(String::from(value)) },
            _ => if self.next().literal().len() > 0 {
                Some(String::from(self.next().literal()))
            } else {
                None
            },
        };
        match name {
            Some(value) => { self.advance(); (true, value) },
            None => { (false, String::new()) },
        }
    }

    fn try_read_interval<T: helpers::ValueReadCheck + helpers::WithInterval>(&mut self, result: &mut T) -> bool {
        if self.expect(&T::token_checker) {
            result.set_min(self);
            if !self.expect(&Token::is_special(SpecialToken::Interval)) {
                self.panic_expect("..")
            }
            if self.expect(&T::token_checker) {
                result.set_max(self);
            }
            true
        } else if self.expect(&Token::is_special(SpecialToken::Interval)) {
            self.advance();
            if self.expect(&T::token_checker) {
                result.set_max(self);
            }
            true
        } else {
            false
        }
    }

    fn try_read_enum<T: helpers::ValueReadCheck 
                + helpers::WithEnum 
                + ObjectBase>(&mut self, output: &mut T) -> bool {
        if self.expect(&Token::is_special(SpecialToken::Enum)) {
            if self.expect(&Token::is_special(SpecialToken::LBrace)) {
                while self.expect(&T::token_checker) {
                    output.enum_add_value(self);
                    self.expect(&Token::is_special(SpecialToken::Comma));
                }
                if !self.expect(&Token::is_special(SpecialToken::RBrace)) {
                    self.panic_expect(&(String::from("} or ") + T::expected()));
                }
                return true;
            } else {
                self.panic_expect(&(String::from("{")));
                return false;
            }
        } else {
            return false;
        }
    }

    fn read_value_nocheck<T: helpers::ValueReadCheck + ObjectBase>(&mut self, output: &mut T) {
        if !output.is_array() {
            if !self.expect(&T::token_checker) {
                self.panic_expect(T::expected());
            }
            output.read_value(self);
        } else {
            if !self.expect(&Token::is_special(SpecialToken::LBracket)) {
                self.panic_expect("[");
            }
            self.try_read_array(output);
        }
    }

    fn read_value<T: helpers::ValueReadCheck + ObjectBase>(&mut self, output: &mut T) {
        if self.expect(&Token::is_special(SpecialToken::Equal)) || 
            self.expect(&Token::is_special(SpecialToken::Colon)) {
            self.read_value_nocheck(output);
        }
    }

    fn try_read_array<T: helpers::ValueReadCheck>(&mut self, output: &mut T) {
        while self.expect(&T::token_checker) {
            output.read_value(self);
            self.expect(&Token::is_special(SpecialToken::Comma));
        }
        if !self.expect(&Token::is_special(SpecialToken::RBracket)) {
            self.panic_expect(&(String::from("] or ") + T::expected()));
        }
    }

    fn parse_number<T: helpers::ValueReadCheck 
            + ObjectBase 
            + helpers::WithEnum
            + helpers::WithInterval>(&mut self, mut result: T) -> T {
        result = self.parse_begin(result);

        while self.try_read_interval(&mut result) ||
              self.try_read_enum(&mut result) {}

        self.read_value(&mut result);
        return result;
    }

    pub fn parse_integer(&mut self) -> IntegerType {
        self.parse_number(IntegerType::new())
    } 
    
    pub fn parse_floating(&mut self) -> FloatingType {
        self.parse_number(FloatingType::new())
    }

    pub fn parse_boolean(&mut self) -> BooleanType {
        let mut result = self.parse_begin(BooleanType::new());
        self.read_value(&mut result);
        return result;
    }

    pub fn parse_string(&mut self) -> StringType {
        let mut result = self.parse_begin(StringType::new());
        self.try_read_enum(&mut result);
        self.read_value(&mut result);
        result
    }

    pub fn parse_object(&mut self) -> ObjectType {
        let mut result = self.parse_begin(ObjectType::new());
        if self.expect(&Token::is_special(SpecialToken::LBrace)) {
            while !self.expect(&Token::is_special(SpecialToken::RBrace)) {
                let element = self.parse_field();
                if result.has_field(element.name()) {
                    panic!("Field '{}' is already defined in onject.", element.name());
                }
                result.add_field(element);
                if self.expect(&Token::is_special(SpecialToken::Semicolon)) ||
                    self.expect(&Token::is_special(SpecialToken::Comma)) {}
            } 
            self.read_value(&mut result);
        }
        return result;
    }

    // any is a very special case
    pub fn parse_any(&mut self) -> AnyType {
        let mut result = AnyType::new();
        if self.expect(&Token::is_special(SpecialToken::Equal)) ||
        self.expect(&Token::is_special(SpecialToken::Colon)) {
            result.add_value(self.guess_element());
        } else {
            self.panic_expect("= or :")
        }
        result

    }

    fn parse_value_for<T: helpers::ValueReadCheck + ObjectBase>(&mut self, mut value: T) -> T {
        self.read_value_nocheck(&mut value);
        value
    }

    fn read_any_array(&mut self) -> Element {
        let mut new_any = AnyType::new_array();
        while !self.expect(&Token::is_special(SpecialToken::RBracket)) {
            new_any.add_value(self.guess_element());
            if self.expect(&Token::is_special(SpecialToken::Comma)) {}
        }
        return Element::Any(new_any);
    }
 
    fn guess_object(&mut self) -> ObjectType {
        let mut next = ObjectType::new();
        while !self.expect(&Token::is_special(SpecialToken::RBrace)) {
            let (found, field_name) = self.read_name();
            if !found {
                if Token::is_special(SpecialToken::RBrace)(self.next().token()) {
                    break;
                } else {
                    self.panic_expect("ident, string or }");
                }
            }

            if self.expect(&Token::is_special(SpecialToken::Equal)) || 
                self.expect(&Token::is_special(SpecialToken::Colon)) {}

            let opts = Options::new();
            next.add_field(FieldType::new(field_name, self.guess_element(), opts));
            self.expect(&Token::is_special(SpecialToken::Comma));
        }
        return next;
    }

    fn guess_number(&mut self) -> Element {
        let bu = self.backup();
        self.advance();
        match self.next().token() {
            Token::Integer(_) => { 
                self.restore(&bu); 
                Element::Integer(self.parse_value_for(IntegerType::new())) 
            },
            Token::Floating(_) => { 
                self.restore(&bu); 
                Element::Floating(self.parse_value_for(FloatingType::new())) 
            },
             _ => { self.panic_expect("number value"); Element::None },
        }
    }

    fn guess_element(&mut self) -> Element {
        match &self.next().token() {
            Token::Integer(_) => { Element::Integer(self.parse_value_for(IntegerType::new())) },
            Token::Floating(_) => Element::Floating(self.parse_value_for(FloatingType::new())),
            Token::Boolean(_) => Element::Boolean(self.parse_value_for(BooleanType::new())),
            Token::String(_) => Element::String(self.parse_value_for(StringType::new())),
            Token::Special(v) => match v {
                SpecialToken::LBrace => {
                    self.advance(); 
                    Element::Object(self.guess_object())
                },
                SpecialToken::LBracket => { // any array 
                    self.advance();
                    self.read_any_array()
                }, 
                SpecialToken::Minus 
                    | SpecialToken::Plus=> { // numbers 
                    self.guess_number()
                }, 
                SpecialToken::Null => { // null
                    self.advance();
                    Element::Any(AnyType::new())
                } 
                _ => { self.panic_expect("valid data"); Element::None },
            }
            _ => { self.panic_expect("valid data"); Element::None },
        }
    }

    fn try_read_options(&mut self) -> Options {
        let mut result = Options::new();
        if self.expect(&Token::is_special(SpecialToken::LParen)) {
            while !self.expect(&Token::is_special(SpecialToken::RParen)) {
                let (found, name) = self.read_name();
                if !found {
                    self.panic_expect("ident or string");
                }

                if self.expect(&Token::is_special(SpecialToken::Equal)) || self.expect(&Token::is_special(SpecialToken::Colon)) {
                    let element = self.guess_element();
                    match element {
                        Element::None => { result.add(&name, Element::Boolean(BooleanType::from(true))) },
                        _ => { result.add(&name, element) }
                    } 
                } else {
                    result.add(&name, Element::Boolean(BooleanType::from(true)))
                }

                self.expect(&Token::is_special(SpecialToken::Comma));
            }
        }
        result
    }

    pub fn parse_field(&mut self) -> FieldType {
        let (_, name) = self.read_name();
        let opts = self.try_read_options();
        if name.len() > 0 && !self.expect(&Token::is_special(SpecialToken::Colon)) {
            self.panic_expect(":");
        }
        self.advance();
        let element = match &self.current().token() {
            Token::Type(name) => match name {
                TypeName::TypeString => Element::String(self.parse_string()),
                TypeName::TypeInteger => Element::Integer(self.parse_integer()),
                TypeName::TypeFloating => Element::Floating(self.parse_floating()),
                TypeName::TypeBoolean => Element::Boolean(self.parse_boolean()),
                TypeName::TypeObject => Element::Object(self.parse_object()),
                TypeName::TypeAny => Element::Any(self.parse_any()),
            },
            _ => { self.panic_current("typename"); Element::None }
        };
        FieldType::new(name, element, opts)
    }
}
