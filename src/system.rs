use crate::parse::BuildConfiguration;
use anyhow::{Ok, Result};
use std::{path::Path, process::Command};

pub struct BuildSystem {
    pub path: String,
    // pub contents: String,
    pub build_file: BuildConfiguration,
}

impl BuildSystem {
    pub fn new(path: &String) -> Result<Self> {
        let place = std::fs::canonicalize(Path::new(path).join("build.toml"))?;
        let contents = std::fs::read_to_string(&place)?;
        println!(
            "Parsing {}",
            place.to_str().unwrap_or("<failed to get file name>")
        );
        let build_file = toml::from_str::<BuildConfiguration>(&contents)?;
        Ok(Self {
            path: path.clone(),
            // contents,
            build_file,
        })
    }

    pub fn construct(&self) -> Result<()> {
        let name = &self.build_file.project.name;
        let version = &self.build_file.project.version;
        // let language = self.build_file.project.language.clone();
        // let authors = self.build_file.project.authors.clone();
        let b = std::fs::canonicalize(&self.path)?;
        let path = b.to_str().unwrap_or("");
        println!("  Compiling {} v{} ({})", name, version, path);
        let compiler = &self.build_file.build.compiler;
        let sources = self
            .build_file
            .build
            .source
            .iter()
            .map(|s| {
                if !s.starts_with("/") {
                    b.join(s)
                        .to_str()
                        .expect("Couldn't convert file name")
                        .to_string()
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
        let includes = &self
            .build_file
            .build
            .include
            .iter()
            .map(|m| "-I".to_string() + &b.join(m).to_str().expect("Failed to do get `include`"))
            .collect::<Vec<String>>()
            .join(" ");
        let mut command = Command::new(&compiler);
        // // let com = command.args([&includes, &sources]);
        // // com.output().expect("Failed to compile your shit");
        // // println!("{command}");
        // command.arg("-c").arg("9ii01");
        // command.output()?;
        let out = format!("-o{}", b.join(name.clone() + ".out".into()).to_str().unwrap());
        command.arg(&sources).arg(&includes).arg(&out).spawn()?;
        Ok(())
    }
}
