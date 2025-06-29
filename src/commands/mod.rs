pub mod get;
pub mod list;
pub mod remove;
pub mod root;
pub mod create;
pub mod switch;
pub mod clean;
pub mod status;

use anyhow::Result;
use crate::cli::Commands;

pub fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Get { url, branch } => get::execute(url, branch),
        Commands::List => list::execute(),
        Commands::Remove { path } => remove::execute(path),
        Commands::Root => root::execute(),
        Commands::Create => create::execute(),
        Commands::Switch { repo, branch } => switch::execute(repo, branch),
        Commands::Clean => clean::execute(),
        Commands::Status => status::execute(),
    }
}