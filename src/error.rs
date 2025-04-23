use std::io;

use thiserror::Error;

use crate::Rule;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("entrypoint must be a YANG module, not submodule")]
    InvalidParserEntrypoint,

    #[error("invalid YANG file")]
    ParseError(#[from] pest::error::Error<Rule>),

    #[error("invalid input file")]
    InvalidFile(#[from] io::Error),

    #[error("included file has to be a submodule, not module: {0}")]
    InvalidInclude(String),
    
    #[error("imported file has to be a module, not submodule: {0}")]
    InvalidImport(String),
}
