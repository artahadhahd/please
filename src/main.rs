mod cli;
mod gen;
mod parse;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use gen::create_new;
use parse::Redirect;

fn main() -> Result<()> {
    let cmds = Cli::parse();
    if cmds.new.is_some() {
        create_new(cmds)?;
        return Ok(());
    }
    let build_file = std::fs::read_to_string("Build.toml").expect("no Build.toml in current directory");
    let project = Redirect::parse(build_file)?;
    project.run(&cmds.initial)?;
    Ok(())
}
