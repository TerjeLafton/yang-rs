#![allow(dead_code)]

mod ast;
pub mod parse;
pub mod parser;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "yang.pest"]
pub struct YangModule;
