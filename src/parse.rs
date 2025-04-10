use pest::iterators::Pair;
use crate::Rule;

/// Trait for parsing YANG elements from pest pairs
pub trait Parse: Sized {
    /// Parse an instance of Self from a pest Pair
    fn parse(pair: Pair<Rule>) -> Self;
}

/// Helper functions for common parsing operations
pub mod helpers {
    use pest::iterators::Pair;
    use crate::Rule;

    /// Parse a string value from a pair
    pub fn parse_string(input: Pair<Rule>) -> String {
        let value = input
            .into_inner()
            .next()
            .expect("string to always have the string value as the only child");

        match value.as_rule() {
            Rule::string => parse_string(value),
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

    /// Parse a boolean value from a pair
    pub fn parse_boolean(input: Pair<Rule>) -> bool {
        let value = input
            .into_inner()
            .next()
            .expect("boolean to always have the bool value as the only child");

        match value.as_str() {
            "true" => true,
            "false" => false,
            _ => unreachable!("Unexpected value: {:?}", value.as_str()),
        }
    }

    /// Parse an integer value from a pair
    pub fn parse_integer(input: Pair<Rule>) -> i64 {
        input
            .into_inner()
            .next()
            .expect("integer to always have the integers value as the only child")
            .as_str()
            .parse()
            .expect("integer value to always be a valid integer")
    }
}