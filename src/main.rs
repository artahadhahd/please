mod cli;
mod gen;
mod parse;

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
    let project = Redirect::parse(include_str!("build.toml").into())?;
    project.run(&cmds.initial)?;
    Ok(())
}
