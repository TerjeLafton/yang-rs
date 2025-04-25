use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = yang_parser::parse("yang-parser/examples/module-a.yang")?;
    dbg!(file);
    Ok(())
}
