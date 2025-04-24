use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the file and resolve references
    let file = yang_rs::parse("module-a.yang")?;

    dbg!(file);

    Ok(())
}
