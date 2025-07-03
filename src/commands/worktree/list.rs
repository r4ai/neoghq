use anyhow::Result;

pub fn execute() -> Result<()> {
    println!("Listing all worktrees:");
    println!("worktree list functionality not yet implemented");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let result = execute();
        assert!(result.is_ok());
    }
}
