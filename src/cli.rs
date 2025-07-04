use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "neoghq")]
#[command(about = "Git Worktree-Based Repository Manager")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Repository operations
    Repo {
        #[command(subcommand)]
        command: RepoCommands,
    },
    /// Worktree operations
    Worktree {
        #[command(subcommand)]
        command: WorktreeCommands,
    },
    /// Show neoghq root directory path
    Root,
}

#[derive(Subcommand)]
pub enum RepoCommands {
    /// Clone repository and create default branch worktree
    Clone { url: String },
    /// Create a new repository and initialize worktree
    Create {
        /// Repository in format 'user/repo' or full URL
        repo: String,
        /// Specify worktree name (default: main)
        #[arg(long)]
        worktree: Option<String>,
    },
    /// Navigate to repository directory
    Switch {
        /// Repository in format 'user/repo'
        repo: String,
        /// Switch to specific worktree
        #[arg(long)]
        worktree: Option<String>,
    },
    /// List all managed repositories
    List {
        /// Show worktrees for each repository
        #[arg(long)]
        show_worktrees: bool,
    },
}

#[derive(Subcommand)]
pub enum WorktreeCommands {
    /// Create worktree from default branch
    Create { branch: String },
    /// Navigate to specified worktree
    Switch { branch: String },
    /// Remove worktree
    #[command(alias = "rm")]
    Remove { branch: String },
    /// Remove worktrees merged to default branch
    Clean,
    /// Show status of all worktrees
    Status,
    /// List all managed worktrees
    List,
}
