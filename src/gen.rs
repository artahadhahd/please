use crate::cli::{Cli, Lang};
use anyhow::Result;
use std::io::Write;

pub fn create_new(cli: Cli) -> Result<()> {
    if let Some(name) = cli.new {
        std::fs::create_dir(&name)?;
        let project_root = std::fs::canonicalize(&name)?;
        let source_directory = project_root.join("src");
        std::fs::create_dir(&source_directory)?;
        let extension = if let Some(lang) = cli.language {
            // fuck this
            match lang {
                Lang::c => "c",
                Lang::cpp => "cpp",
            }
        } else {
            "c"
        };
        let main = source_directory.join(&format!("{name}.{extension}"));
        let mut file = std::fs::File::create(main)?;
        file.write_all(match extension {
            "cpp" => include_bytes!("templates/cpp"),
            _ => include_bytes!("templates/c"),
        })?;
        let build_file = project_root.join("build.toml");
        let mut build_file = std::fs::File::create(build_file)?;
        build_file.write_all(
            format!(
                r#"
[project]
name = "{name}"
version = "0.1"
type = "app"

[build]
sources = ["src/{name}.{extension}"]"#
            )
            .as_bytes(),
        )?;
    }
    Ok(())
}
