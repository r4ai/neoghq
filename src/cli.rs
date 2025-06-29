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
    #[command(alias = "clone")]
    Get {
        url: String,
        #[arg(help = "Branch name")]
        branch: Option<String>,
    },
    List,
    #[command(alias = "rm")]
    Remove {
        path: String,
    },
    Root,
    Create,
    Switch {
        repo: String,
        branch: String,
    },
    Clean,
    Status,
}