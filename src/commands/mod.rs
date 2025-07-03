pub mod clean;
pub mod create;
pub mod get;
pub mod list;
pub mod remove;
pub mod root;
pub mod status;
pub mod switch;

use crate::{cli::Commands, config::Config};
use anyhow::Result;

pub fn execute_command(command: Commands, config: Config) -> Result<()> {
    match command {
        Commands::Get { url, branch } => get::execute(config, url, branch),
        Commands::List => list::execute(),
        Commands::Remove { path } => remove::execute(path),
        Commands::Root => root::execute(),
        Commands::Create => create::execute(),
        Commands::Switch { repo, branch } => switch::execute(repo, branch),
        Commands::Clean => clean::execute(),
        Commands::Status => status::execute(),
    }
}
