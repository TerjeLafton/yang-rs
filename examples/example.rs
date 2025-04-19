use std::error::Error;
use yang_rs::parser::YangParser;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the file and resolve references
    let file = YangParser::parse_file("examples/example.yang")?;

    // Print out the resolved AST
    println!("{:#?}", file);

    Ok(())
}
