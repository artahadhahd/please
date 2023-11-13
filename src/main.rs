mod app;
mod cli;
mod parse;
mod system;

use anyhow::Result;
use app::App;
use clap::Parser;
use cli::Cli;
use system::BuildSystem;

fn main() -> Result<()> {
    let cli = Cli::parse().get_result();
    let app = App::new(cli)?;
    app.run()?;
    Ok(())
}
