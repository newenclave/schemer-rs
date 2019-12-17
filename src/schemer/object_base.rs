#![allow(unused)]

use super::objects::*;
use super::helpers::*;

pub trait ObjectBase {
    fn new() -> Self;
    fn is_array(&self) -> bool;
    fn is_default(&self) -> bool;
    fn set_array(&mut self);
    fn clone(&self) -> Self;
    fn value_to_string(&self) -> String;
}
