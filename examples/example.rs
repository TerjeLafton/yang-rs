use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = yang_rs::parse("examples/module-a.yang")?;
    dbg!(file);
    Ok(())
}
