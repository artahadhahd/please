// use std::collections::HashMap;
use std::{fmt::Debug, fs::canonicalize, path::PathBuf};

use crate::cli::Command;
use anyhow::{Ok, Result};
use colored::Colorize;
use serde::Deserialize;
use std::{process, fs};

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Default)]
pub enum ProjectType {
    #[default]
    app,
    lib,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Default)]
pub enum Languages {
    #[default]
    c,
    cpp,
    cxx,
    cc,
    #[serde(rename = "c++")]
    Cpp,
}

#[derive(Debug)]
pub enum CompilationError {
    Compiling(String),
    Linking(String),
}

use std::fmt;
impl std::error::Error for CompilationError {}
impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CompilationError::*;
        match self {
            Compiling(name) => write!(f, "{} '{name}'", "Failed to compile".bold().red()),
            Linking(name) => write!(f, "{} {name}", "Failed to link".bold().red()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Output {
    pub dir: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub r#type: ProjectType,
    // pub language: Option<Vec<Languages>>,
}

#[derive(Deserialize, Debug)]
struct DummyRoot {
    project: Project,
}

#[derive(Deserialize, Debug)]
pub struct Dependencies {
    pub local: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Build {
    pub compiler: String,
    pub sources: Vec<String>,
    pub includes: Option<Vec<String>>,
    pub warnings: Option<u8>,
    pub objects: Option<bool>,
    pub bin: Option<String>, // The name of the output
}

pub enum Redirect {
    App(AppRoot),
    Lib(LibRoot),
}

#[derive(Deserialize, Debug)]
pub struct AppRoot {
    pub project: Project,
    pub build: Build,
    pub dependencies: Option<Dependencies>,
}

impl AppRoot {
    pub fn run(&self, command: &Command) -> Result<()> {
        match command {
            Command::build => self.build_project()?,
            // Command::run => {
            //     self.build_project()?;
            //     std::process::Command::new(&format!("./{}", self.get_output_name())).status()?;
            // },
            _ => todo!(),
        }
        Ok(())
    }

    fn build_project(&self) -> Result<()> {
        println!("{} {} v{}", "  Building".green().bold(), &self.project.name, &self.project.version);
        let build_sources = &self.build.sources;
        if self.build.objects.unwrap_or(false) {
            let objects = self.compilation_stage(&build_sources)?;
            self.link_from(&objects)?;
        } else {
            self.link_from(build_sources)?;
        }
        Ok(())
    }

    fn get_output_name(&self) -> String {
        self.build.bin.clone().unwrap_or(self.project.name.clone())
    }

    fn link_from(&self, sources: &Vec<String>) -> Result<()> {
        let mut compiler = process::Command::new(&self.build.compiler);
        for source in sources.iter() {
            compiler.arg(source);
        }
        compiler.arg("-o").arg(&self.get_output_name());
        let status = compiler.status()?;
        if !status.success() {
            return Err(CompilationError::Linking(self.project.name.to_owned()).into());
        }
        Ok(())
    }

    /* returns a vec with format -I, <include>  */
    fn get_includes(&self) -> Option<Vec<String>> {
        let mut out: Vec<String> = vec![];
        if self.build.includes.is_none() {
            return None;
        }
        for include in self.build.includes.as_ref().unwrap() {
            out.push("-I".into());
            out.push(include.clone());
        }
        Some(out)
    }

    // TODO: this design is fucking terrible, what if user wants pedantic and all?
    // TODO: MSVC compiler flag support?
    fn get_warnings(&self) -> Option<Vec<String>> {
        match self.build.warnings.unwrap_or(0u8) {
            0 => None,
            1 => Some(Vec::from(["-Wall".into()])),
            2 => Some(Vec::from(["-Wall".into(), "-Wextra".into()])),
            3 => Some(Vec::from([
                "-Wall".into(),
                "-Wextra".into(),
                "-Wpedantic".into(),
            ])),
            _ => Some(Vec::from([
                "-Wall".into(),
                "-Wextra".into(),
                "-Wpedantic".into(),
                "-Werror".into(),
            ])),
        }
    }

    fn compilation_stage(&self, sources: &Vec<String>) -> Result<Vec<String>> {
        let mut out: Vec<String> = vec![];
        for file in sources.iter() {
            let mut compiler = std::process::Command::new(&self.build.compiler);
            let mut output = canonicalize(PathBuf::from(file))?;
            let _metadata = fs::metadata(&output)?;
            // dbg!(metadata);
            output.set_extension("o");
            out.push(output.to_str().unwrap().to_string());
            compiler.arg("-c").arg(file).arg("-o").arg(output);
            let warnings = self.get_warnings();
            if warnings.is_some() {
                compiler.args(&warnings.unwrap());
            }
            let includes = self.get_includes();
            if includes.is_some() {
                compiler.args(&includes.unwrap());
            }
            let status = compiler.status()?;
            let status = status.success();
            if !status {
                return Err(CompilationError::Compiling(self.project.name.to_owned()).into());
            }
        }
        Ok(out)
    }
}

#[derive(Deserialize, Debug)]
pub struct LibRoot {
    pub project: Project,
    pub build: Build,
    pub output: Output,
}

impl LibRoot {
    pub fn run(&self, _cmd: &Command) {}
}

// I don't know what to call this?
impl Redirect {
    pub fn parse(inp: String) -> Result<Self> {
        let processed: DummyRoot = toml::from_str(&inp)?;
        Ok(match processed.project.r#type {
            ProjectType::app => Redirect::App(toml::from_str::<AppRoot>(&inp)?),
            ProjectType::lib => Redirect::Lib(toml::from_str::<LibRoot>(&inp)?),
        })
    }

    pub fn run(&self, cmd: &Option<Command>) -> Result<()> {
        let cmd: Command = match cmd {
            None => Command::run,
            Some(command) => command.to_owned(),
        };
        match self {
            Self::App(app) => app.run(&cmd)?,
            Self::Lib(lib) => lib.run(&cmd),
        }
        Ok(())
    }
}
