/*
 * @Author: your name
 * @Date: 2021-04-10 17:58:37
 * @LastEditTime: 2021-05-09 12:29:37
 * @LastEditors: your name
 * @Description: In User Settings Edit
 * @FilePath: \zisvalidator\zisvalidator_derive\src\symbol.rs
 */
use std::fmt::{self, Display};
use syn::{Ident, Path};

#[derive(Debug)]
pub struct Symbol(&'static str);

pub const VALIDATE: Symbol = Symbol("validate");
pub const CUSTOM: Symbol = Symbol("custom");
pub const RANGE: Symbol = Symbol("range");
pub const SCHEMA: Symbol = Symbol("schema");
pub const MESSAGE: Symbol = Symbol("message");
pub const SEQ_RANGE: Symbol = Symbol("seq_range");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
