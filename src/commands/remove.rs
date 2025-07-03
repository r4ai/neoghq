use anyhow::Result;

pub fn execute(path: String) -> Result<()> {
    println!("Remove command: {path}");
    Ok(())
}
