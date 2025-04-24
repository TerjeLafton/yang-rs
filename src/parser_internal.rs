use pest_derive::Parser;

/// Initializer for Pest to generate the Rules from the grammar.
#[derive(Parser)]
#[grammar = "yang.pest"]
pub struct YangModule;
