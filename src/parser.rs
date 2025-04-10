use std::{collections::HashMap, fs, path::Path};

use pest::{error::Error, Parser};

use crate::{ast::*, parse::Parse, Rule, YangModule};

pub struct YangParser {
    pub groupings: HashMap<String, Grouping>,
    pub type_defs: HashMap<String, TypeDef>,
}

impl YangParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<YangFile, Error<Rule>> {
        let content = fs::read_to_string(path).expect("file to be available");
        Self::parse(&content)
    }

    fn parse(input: &str) -> Result<YangFile, Error<Rule>> {
        let pair = YangModule::parse(Rule::file, input)?
            .next()
            .expect("a yang file to always include a module");

        Ok(YangFile::parse(pair))
    }
}
