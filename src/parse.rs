#![allow(dead_code, unused_variables)]
use std::path::PathBuf;

use crate::cli::Command;
use anyhow::{Result, Ok};
use serde::Deserialize;

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

#[allow(dead_code)]
impl Languages {
    pub fn to_extension(&self) -> String {
        match self {
            Self::c => "c",
            Self::cpp | Self::cxx | Self::Cpp | Self::cc => "cpp",
        }
        .into()
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

#[derive(Deserialize, Debug, Default)]
pub struct Build {
    pub compiler: String,
    pub sources: Vec<String>,
    pub includes: Option<Vec<String>>,
    pub warnings: Option<u8>,
    pub object: Option<bool>,
}

pub enum Redirect {
    App(AppRoot),
    Lib(LibRoot),
}

#[derive(Deserialize, Debug)]
pub struct AppRoot {
    pub project: Project,
    pub build: Build,
}

impl AppRoot {
    pub fn run(&self, command: &Command) -> Result<()> {
        match command {
            Command::build => self.build_project()?,
            _ => todo!(),
        }
        Ok(())
    }


    fn build_project(&self) -> Result<()> {
        let build_sources = &self.build.sources;
        if self.build.object.unwrap_or(false) {
            self.compilation_stage(build_sources)?;
            linking_stage(build_sources);
        } else {

        }
        Ok(())
    }

    fn compilation_stage(&self, sources: &Vec<String>) -> Result<()> {
        for file in sources.iter() {
            let mut compiler = std::process::Command::new(&self.build.compiler);
            let mut output = PathBuf::from(file);
            output.set_extension("o");
            dbg!(&output);
            compiler.arg("-c").arg(file).arg("-o").arg(output);
            compiler.spawn()?;
        }
        dbg!(sources);
        Ok(())
    }
}


fn linking_stage(sources: &Vec<String>) {

}

#[derive(Deserialize, Debug)]
pub struct LibRoot {
    pub project: Project,
    pub build: Build,
    pub output: Output,
}

impl LibRoot {
    pub fn run(&self, cmd: &Command) {}
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
