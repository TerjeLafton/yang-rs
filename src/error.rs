use thiserror::Error;

use crate::Rule;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("entrypoint must be a valid YANG module")]
    InvalidEntrypoint,

    #[error("invalid YANG file")]
    InvalidFile(#[from] pest::error::Error<Rule>),
}
