#![allow(dead_code)]

mod error;
mod ir;
pub mod parser;
mod resolver;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "yang.pest"]
pub struct YangModule;
