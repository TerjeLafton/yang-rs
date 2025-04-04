use std::{fs, path::Path};

use pest::{error::Error, Parser};

use crate::{ast::*, Rule, YangModule};

pub struct YangParser;

impl YangParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<YangFile, Error<Rule>> {
        let content = fs::read_to_string(path).expect("Failed to read file");
        Self::parse_str(&content)
    }

    pub fn parse_str(input: &str) -> Result<YangFile, Error<Rule>> {
        let pairs = YangModule::parse(Rule::file, input)?;

        let module = Module {
            name: "Real".into(),
            yang_version: None,
            namespace: "test".into(),
            prefix: "test".into(),
            imports: vec![],
            includes: vec![],
            meta: MetaInfo::default(),
            revisions: vec![],
            body: vec![],
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::module => return Ok(YangFile::Module(module)),
                Rule::submodule => continue,
                Rule::EOI => continue,
                _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
            }
        }

        unreachable!("Expected to find a module or submodule")
    }
}
