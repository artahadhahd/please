#![allow(unused_braces)]
use crate::parse::{BuildConfiguration, Language};
use anyhow::{Ok, Result};
use colored::Colorize;
use std::{
    path::PathBuf,
    process::{exit, Command},
};

pub struct BuildSystem {
    pub path: PathBuf,
    pub build_file: BuildConfiguration,
    pub failed: bool,
}

fn absolute_or_relative(m: &String, b: &PathBuf) -> String {
    if m.starts_with("/") {
        m.to_string()
    } else {
        b.join(m).to_str().unwrap().into()
    }
}

impl BuildSystem {
    pub fn new(path: &String) -> Result<Self> {
        let path = std::fs::canonicalize(path)?;
        let place = path.join("build.toml");
        let contents = std::fs::read_to_string(&place)?;
        println!(
            "Parsing {}",
            place.to_str().unwrap_or("<failed to get file name>")
        );
        let build_file = toml::from_str::<BuildConfiguration>(&contents)?;
        Ok(Self {
            path,
            // contents,
            build_file,
            failed: false,
        })
    }

    pub fn construct(&self) -> Result<()> {
        let name = &self.build_file.project.name;
        let version = &self.build_file.project.version;
        // let language = self.build_file.project.language.clone();
        // let authors = self.build_file.project.authors.clone();
        let path = self.path.to_str().unwrap_or("");
        let compiler = &self.build_file.build.compiler;
        let lang: String = match &self.build_file.project.language {
            Some(lang) => match lang {
                Language::Language(l) => l.to_owned(),
                Language::Languages(langs) => langs.join(", "),
            },
            _ => match compiler.as_str() {
                "gcc" | "clang" => "c".into(),
                "g++" | "clang++" => "cpp".into(),
                _ => "?".into(),
            },
        };
        println!(
            "   {} {} v{} ({}) [{}]",
            "Compiling".green().bold(),
            name,
            version,
            path,
            lang
        );
        let sources = self.get_sources().join(" ");
        let includes = self.get_includes(true).join(" ");
        if self.build_file.dependencies.is_some() {
            let _deps = self.build_file.dependencies.as_ref().unwrap();
        }
        let use_objects = match self.build_file.build.objects {
            Some(value) => value,
            _ => false,
        };
        let objs = if use_objects {
            self.all_object(&self.get_sources())
        } else {
            vec![]
        };
        dbg!(objs);
        let mut command = Command::new(&compiler);
        let out = format!(
            "-o{}",
            self.path
                .join(format!("{name}.out"))
                .to_str()
                .expect("Bad file name")
        );

        let mut command = command.arg(&sources).arg(&includes).arg(&out).spawn()?;
        let status = command.wait()?;
        if !status.success() {
            println!("{} for {name}", "Compilation failed".red().bold());
            exit(1);
        }
        Ok(())
    }

    fn get_sources(&self) -> Vec<String> {
        self.build_file
            .build
            .source
            .iter()
            .map(|s| absolute_or_relative(s, &self.path))
            .collect::<Vec<String>>()
    }

    fn get_includes(&self, formatted: bool) -> Vec<String> {
        self.build_file
            .build
            .include
            .iter()
            .map(|m| {
                format!(
                    "{}{}",
                    if formatted { "-I" } else { "" },
                    absolute_or_relative(m, &self.path)
                )
            })
            .collect::<Vec<String>>()
    }

    fn all_object(&self, sources: &Vec<String>) -> Vec<String> {
        let mut out: Vec<String> = vec![];
        for source in sources.iter() {
            out.push(to_object(source));
        }
        out
    }

    #[allow(dead_code)]
    fn invoke_git() {}
}

fn to_object(source: &String) -> String {
    let mut buh = PathBuf::from(source);
    buh.set_extension("o");
    let s = buh.to_str().expect(&format!("Couldn't get object file for source {}", source));
    s.to_string()
}