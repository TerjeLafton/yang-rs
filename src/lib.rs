#![allow(dead_code)]

mod ast;
mod error;
pub mod parser;
mod resolver;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "yang.pest"]
pub struct YangModule;
