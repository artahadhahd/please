mod cli;
mod parse;

use std::io::Write;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Lang};
use parse::Redirect;

fn create_new(cli: Cli) -> Result<()> {
    if let Some(name) = cli.new {
        std::fs::create_dir(&name)?;
        let project_root = std::fs::canonicalize(&name)?;
        let source_directory = project_root.join("src");
        std::fs::create_dir(&source_directory)?;
        let extension = if let Some(lang) = cli.language { // fuck this
            match lang {
                Lang::c => "c",
                Lang::cpp => "cpp",
            }
        } else {
            "c"
        };
        let main = source_directory.join(&format!("{name}.{extension}"));
        let mut file =
            std::fs::File::create(main)?;
        file.write_all(match extension {
            "cpp" => include_bytes!("templates/cpp"),
            _ => include_bytes!("templates/c"),
        })?;
        let build_file = project_root.join("build.toml");
        let mut build_file = std::fs::File::create(build_file)?;
        build_file.write_all(format!(
            r#"[project]
name = "{name}"
version = "0.1"
type = "app"

[build]
sources = ["src/{name}.{extension}"]"#).as_bytes())?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let cmds = Cli::parse();
    if cmds.new.is_some() {
        create_new(cmds)?;
        return Ok(());
    }
    let project = Redirect::parse(include_str!("build.toml").into())?;
    project.run(&cmds.initial);
    Ok(())
}
