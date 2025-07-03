use anyhow::Result;

pub fn execute(repo: String, branch: String) -> Result<()> {
    println!("Switch command: {repo} {branch}");
    Ok(())
}
