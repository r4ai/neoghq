use anyhow::Result;

pub fn execute(url: String, branch: Option<String>) -> Result<()> {
    println!("Get command: {} {:?}", url, branch);
    Ok(())
}