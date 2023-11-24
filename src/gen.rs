use crate::{cli::{Cli, Lang}, errors::CompilationError};
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::io::Write;

pub fn create_new(cli: Cli) -> Result<()> {
    if let Some(name) = cli.new {
        fs::create_dir(&name)?;
        let project_root = fs::canonicalize(&name)?;
        println!(
            "    {} project '{name}' ({:?})",
            "Creating".green().bold(),
            project_root
        );
        let source_directory = project_root.join("src");
        fs::create_dir(&source_directory)?;
        let extension = cli.language.unwrap_or(Lang::c).to_extension();
        let main = source_directory.join(&format!("main.{extension}"));
        let mut file = fs::File::create(main)?;
        file.write_all(match extension {
            "cpp" => include_bytes!("templates/cpp"),
            _ => include_bytes!("templates/c"),
        })?;
        let bfile = project_root.join("Build.toml");
        let build_file = fs::File::create(&bfile);
        if build_file.is_err() {
            return Err(CompilationError::FailedToCreateFile(bfile.clone()).into());
        }
        let mut build_file = build_file.unwrap();
        build_file.write_all(
            format!(
                r#"# File auto-generated by pwease
[project]
name = "{name}"
version = "0.1"
type = "app"

[build]
#compiler = 
sources = ["src/main.{extension}"]
"#
            )
            .as_bytes(),
        )?;
    }
    Ok(())
}
