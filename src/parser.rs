use std::{fs, path::Path};

use pest::{error::Error, iterators::Pair, Parser};

use crate::{ast::*, Rule, YangModule};

pub struct YangParser;

impl YangParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<YangFile, Error<Rule>> {
        let content = fs::read_to_string(path).expect("Failed to read file");
        Self::parse(&content)
    }

    fn parse(input: &str) -> Result<YangFile, Error<Rule>> {
        let module = YangModule::parse(Rule::file, input)?.next().unwrap();

        match module.as_rule() {
            Rule::module => Ok(YangFile::Module(Self::parse_module(module))),
            _ => unreachable!("Unexpected rule: {:?}", module.as_rule()),
        }
    }

    fn parse_module(input: Pair<Rule>) -> Module {
        let pair = input;
        let mut module = Module::default();

        for child in pair.into_inner() {
            match child.as_rule() {
                // String here always means the name of the module
                Rule::string => module.name = Self::parse_string(child),
                Rule::prefix => module.prefix = Self::parse_single_string_statement(child),
                Rule::namespace => module.namespace = Self::parse_single_string_statement(child),
                Rule::yang_version => {
                    module.yang_version = Some(Self::parse_single_string_statement(child))
                }
                Rule::organization => {
                    module.meta.organization = Some(Self::parse_single_string_statement(child))
                }
                Rule::contact => {
                    module.meta.contact = Some(Self::parse_single_string_statement(child))
                }
                Rule::description => {
                    module.meta.description = Some(Self::parse_single_string_statement(child))
                }
                Rule::reference => {
                    module.meta.reference = Some(Self::parse_single_string_statement(child))
                }
                Rule::revision => {
                    module.revisions.push(Self::parse_revision(child));
                }
                Rule::import => {
                    module.imports.push(Self::parse_import(child));
                }

                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        module
    }

    fn parse_revision(input: Pair<Rule>) -> Revision {
        let mut revision = Revision::default();

        for child in input.into_inner() {
            match child.as_rule() {
                // String here always means the date of the module
                Rule::string => revision.date = Self::parse_string(child),
                Rule::description => {
                    revision.description = Some(Self::parse_single_string_statement(child))
                }
                Rule::reference => {
                    revision.reference = Some(Self::parse_single_string_statement(child))
                }
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        revision
    }

    fn parse_import(input: Pair<Rule>) -> Import {
        let mut import = Import::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => import.module = Self::parse_string(child),
                Rule::prefix => import.prefix = Self::parse_single_string_statement(child),
                Rule::revision_date => {
                    import.revision_date = Some(Self::parse_single_string_statement(child))
                }
                Rule::description => {
                    import.description = Some(Self::parse_single_string_statement(child))
                }
                Rule::reference => {
                    import.reference = Some(Self::parse_single_string_statement(child))
                }
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        import
    }

    fn parse_single_string_statement(input: Pair<Rule>) -> String {
        let value = input.into_inner().next().unwrap();
        Self::parse_string(value)
    }

    fn parse_string(input: Pair<Rule>) -> String {
        let value = input.into_inner().next().unwrap();
        match value.as_rule() {
            Rule::unquoted_string => value.as_str().to_string(),
            Rule::double_quoted_string => {
                let s = value.as_str();
                s[1..s.len() - 1].to_string()
            }
            Rule::single_quoted_string => {
                let s = value.as_str();
                s[1..s.len() - 1].to_string()
            }
            _ => unreachable!("Unexpected rule: {:?}", value.as_rule()),
        }
    }
}
