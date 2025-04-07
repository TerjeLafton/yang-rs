use std::{fs, path::Path};

use pest::{error::Error, iterators::Pair, Parser};

use crate::{ast::*, Rule, YangModule};

pub struct YangParser;

impl YangParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<YangFile, Error<Rule>> {
        let content = fs::read_to_string(path).expect("file to be available");
        Self::parse(&content)
    }

    fn parse(input: &str) -> Result<YangFile, Error<Rule>> {
        let module = YangModule::parse(Rule::file, input)?
            .next()
            .expect("a yang file to always include a module");

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
                Rule::string => module.name = Self::parse_string(child),
                Rule::prefix => module.prefix = Self::parse_string(child),
                Rule::namespace => module.namespace = Self::parse_string(child),
                Rule::yang_version => module.yang_version = Some(Self::parse_string(child)),
                Rule::organization => module.meta.organization = Some(Self::parse_string(child)),
                Rule::contact => module.meta.contact = Some(Self::parse_string(child)),
                Rule::description => module.meta.description = Some(Self::parse_string(child)),
                Rule::reference => module.meta.reference = Some(Self::parse_string(child)),
                Rule::revision => module.revisions.push(Self::parse_revision(child)),
                Rule::import => module.imports.push(Self::parse_import(child)),
                Rule::include => module.includes.push(Self::parse_include(child)),
                Rule::body => module.body.push(Self::parse_body(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        module
    }

    fn parse_body(input: Pair<Rule>) -> SchemaNode {
        // Todo: Need to loop instead of only getting the next node
        // Temporary implementation until Typedef is done, as it is quite big.
        let node = input.into_inner().next().unwrap();

        match node.as_rule() {
            Rule::typedef => Self::parse_typedef(node),
            _ => unreachable!("Unexpected rule: {:?}", node.as_rule()),
        }
    }

    fn parse_typedef(input: Pair<Rule>) -> SchemaNode {
        let mut type_def = TypeDef::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => type_def.name = Self::parse_string(child),
                Rule::type_info => type_def.type_info = Self::parse_type_info(child),
                Rule::units => type_def.units = Some(Self::parse_string(child)),
                Rule::default => type_def.default = Some(Self::parse_string(child)),
                Rule::status => type_def.status = Some(Self::parse_status(child)),
                Rule::description => type_def.description = Some(Self::parse_string(child)),
                Rule::reference => type_def.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        SchemaNode::TypeDef(type_def)
    }

    fn parse_type_info(input: Pair<Rule>) -> TypeInfo {
        let mut type_info = TypeInfo::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => type_info.name = Self::parse_string(child),
                Rule::numberical_restriction => {
                    type_info.type_body = Some(Self::parse_numerical(child))
                }
                Rule::decimal64_specification => {
                    type_info.type_body = Some(Self::parse_decimal(child))
                }
                Rule::string_restriction => {
                    type_info.type_body = Some(Self::parse_string_restriction(child))
                }
                Rule::enum_specification => type_info.type_body = Some(Self::parse_enum(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        return type_info;
    }

    fn parse_string_restriction(input: Pair<Rule>) -> TypeBody {
        let mut length = None;
        let mut patterns = Vec::new();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::length => length = Some(Self::parse_length(child)),
                Rule::pattern => patterns.push(Self::parse_pattern(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        TypeBody::String { length, patterns }
    }

    fn parse_decimal(input: Pair<Rule>) -> TypeBody {
        let mut decimal_node = input.into_inner();
        let fractional_digits = Self::parse_string(
            decimal_node
                .next()
                .expect("decimal's first child always to be a fractional_digits node"),
        );

        match decimal_node.next() {
            Some(range) => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: Some(Self::parse_range(range)),
            },
            None => TypeBody::Decimal64 {
                fraction_digits: fractional_digits,
                range: None,
            },
        }
    }

    fn parse_numerical(input: Pair<Rule>) -> TypeBody {
        TypeBody::Numerical {
            range: Self::parse_range(
                input
                    .into_inner()
                    .next()
                    .expect("numerical to always have range as the only child"),
            ),
        }
    }

    fn parse_enum(input: Pair<Rule>) -> TypeBody {
        let mut enums = Vec::new();
        for enum_child in input.into_inner() {
            let mut enum_value = EnumValue::default();
            for child in enum_child.into_inner() {
                match child.as_rule() {
                    Rule::string => enum_value.name = Self::parse_string(child),
                    Rule::if_feature => enum_value.if_features.push(Self::parse_string(child)),
                    Rule::value => enum_value.value = Some(Self::parse_integer(child)),
                    Rule::status => enum_value.status = Some(Self::parse_status(child)),
                    Rule::description => enum_value.description = Some(Self::parse_string(child)),
                    Rule::reference => enum_value.reference = Some(Self::parse_string(child)),
                    _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
                }
            }
            enums.push(enum_value);
        }

        TypeBody::Enum { enums }
    }

    fn parse_pattern(input: Pair<Rule>) -> Pattern {
        let mut pattern = Pattern::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => pattern.value = Self::parse_string(child),
                Rule::error_message => pattern.error_message = Some(Self::parse_string(child)),
                Rule::error_app_tag => pattern.error_app_tag = Some(Self::parse_string(child)),
                Rule::description => pattern.description = Some(Self::parse_string(child)),
                Rule::reference => pattern.reference = Some(Self::parse_string(child)),
                Rule::modifier => pattern.modifier = Some(
                    child
                        .into_inner()
                        .next()
                        .expect(
                            "modifier to always only have one child which, which is the invert-match string",
                        )
                        .as_str()
                        .to_string(),
                ),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        pattern
    }

    fn parse_length(input: Pair<Rule>) -> Length {
        let mut length = Length::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => length.value = Self::parse_string(child),
                Rule::error_message => length.error_message = Some(Self::parse_string(child)),
                Rule::error_app_tag => length.error_app_tag = Some(Self::parse_string(child)),
                Rule::description => length.description = Some(Self::parse_string(child)),
                Rule::reference => length.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        length
    }

    fn parse_range(input: Pair<Rule>) -> Range {
        let mut range = Range::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => range.value = Self::parse_string(child),
                Rule::error_message => range.error_message = Some(Self::parse_string(child)),
                Rule::error_app_tag => range.error_app_tag = Some(Self::parse_string(child)),
                Rule::description => range.description = Some(Self::parse_string(child)),
                Rule::reference => range.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        range
    }

    fn parse_status(input: Pair<Rule>) -> Status {
        let status = input
            .into_inner()
            .next()
            .expect("status to always have a status_value as the only child");

        match status.as_str() {
            "current" => Status::Current,
            "obsolete" => Status::Obsolete,
            "deprecated" => Status::Deprecated,
            _ => unreachable!("Unexpected status: {:?}", status),
        }
    }

    fn parse_revision(input: Pair<Rule>) -> Revision {
        let mut revision = Revision::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => revision.date = Self::parse_string(child),
                Rule::description => revision.description = Some(Self::parse_string(child)),
                Rule::reference => revision.reference = Some(Self::parse_string(child)),
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
                Rule::prefix => import.prefix = Self::parse_string(child),
                Rule::revision_date => import.revision_date = Some(Self::parse_string(child)),
                Rule::description => import.description = Some(Self::parse_string(child)),
                Rule::reference => import.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        import
    }

    fn parse_include(input: Pair<Rule>) -> Include {
        let mut include = Include::default();

        for child in input.into_inner() {
            match child.as_rule() {
                Rule::string => include.module = Self::parse_string(child),
                Rule::revision_date => include.revision_date = Some(Self::parse_string(child)),
                Rule::description => include.description = Some(Self::parse_string(child)),
                Rule::reference => include.reference = Some(Self::parse_string(child)),
                _ => unreachable!("Unexpected rule: {:?}", child.as_rule()),
            }
        }

        include
    }

    fn parse_integer(input: Pair<Rule>) -> i32 {
        input
            .into_inner()
            .next()
            .expect("integer to always have the integers value as the only child")
            .as_str()
            .parse()
            .expect("integer value to always be a valid integer")
    }

    fn parse_string(input: Pair<Rule>) -> String {
        let value = input
            .into_inner()
            .next()
            .expect("string to always have the string value as the only child");

        match value.as_rule() {
            Rule::string => Self::parse_string(value),
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
