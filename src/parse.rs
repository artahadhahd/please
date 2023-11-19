#![allow(dead_code, unused_variables)]
use crate::cli::Command;
use anyhow::Result;
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
    pub sources: Vec<String>,
    pub includes: Option<Vec<String>>,
    pub warnings: Option<u8>,
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
    pub fn run(&self, command: &Command) {
        match command {
            Command::build => self.build_project(),
            _ => todo!(),
        }
    }

    fn build_project(&self) {}
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

    pub fn run(&self, cmd: &Option<Command>) {
        let cmd: Command = match cmd {
            None => Command::run,
            Some(command) => command.to_owned(),
        };
        match self {
            Self::App(app) => app.run(&cmd),
            Self::Lib(lib) => lib.run(&cmd),
        }
    }
}
