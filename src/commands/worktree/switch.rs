use anyhow::Result;

pub fn execute(branch: String) -> Result<()> {
    println!("Switching to worktree for branch: {branch}");
    println!("worktree switch functionality not yet implemented");
    Ok(())
}
