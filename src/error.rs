use std::io;

use thiserror::Error;

use crate::Rule;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("entrypoint must be a YANG module, not submodule")]
    InvalidParserEntrypoint,
    #[error("invalid YANG file")]
    InvalidFile(#[from] pest::error::Error<Rule>),
    #[error("invalid input file")]
    InvalidInput(#[from] io::Error),
    #[error("included file has to be a submodule, not module: {0}")]
    InvalidInclude(String),
}
