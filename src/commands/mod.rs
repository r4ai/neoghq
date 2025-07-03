pub mod repo;
pub mod root;
pub mod worktree;

use crate::{
    cli::{Commands, RepoCommands, WorktreeCommands},
    config::Config,
};
use anyhow::Result;

pub fn execute_command(command: Commands, config: Config) -> Result<()> {
    match command {
        Commands::Repo { command } => execute_repo_command(command, config),
        Commands::Worktree { command } => execute_worktree_command(command, config),
        Commands::Root => root::execute(),
    }
}

fn execute_repo_command(command: RepoCommands, config: Config) -> Result<()> {
    match command {
        RepoCommands::Clone { url } => repo::clone::execute(config, url, None),
        RepoCommands::Create { url: _ } => {
            // TODO: Implement repo create functionality
            println!("repo create command not yet implemented");
            Ok(())
        }
        RepoCommands::Switch { repo } => repo::switch::execute(repo),
        RepoCommands::List => repo::list::execute(),
    }
}

fn execute_worktree_command(command: WorktreeCommands, _config: Config) -> Result<()> {
    match command {
        WorktreeCommands::Create { branch } => worktree::create::execute(branch),
        WorktreeCommands::Switch { branch } => worktree::switch::execute(branch),
        WorktreeCommands::Remove { branch } => worktree::remove::execute(branch),
        WorktreeCommands::Clean => worktree::clean::execute(),
        WorktreeCommands::Status => worktree::status::execute(),
        WorktreeCommands::List => worktree::list::execute(),
    }
}
