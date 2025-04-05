use std::error::Error;
use yang_rs::parser::YangParser;

fn main() -> Result<(), Box<dyn Error>> {
    let file = YangParser::parse_file("examples/example.yang").unwrap();

    dbg!(file);

    Ok(())
}
