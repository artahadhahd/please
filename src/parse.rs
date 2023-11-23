use anyhow::{Ok, Result};
use colored::Colorize;
use serde::Deserialize;
use std::{
    collections::HashMap, fmt, fmt::Debug, fs, fs::canonicalize, path::PathBuf, process,
    process::Stdio,
};

use crate::{cli::Command, utils::get_extension};

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
    #[allow(dead_code)]
    Dependency(String),
    Cloning(String),
    Ext,
}

impl std::error::Error for CompilationError {}
impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CompilationError::*;
        match self {
            Compiling(name) => write!(f, "{} '{name}'", "Failed to compile".bold().red()),
            Linking(name) => write!(f, "{} {name}", "Failed to link".bold().red()),
            Dependency(dep) => write!(f, "{}: {dep}", "Dependency not found".bold().red()),
            Cloning(dep) => write!(f, "{} '{dep}'", "Failed to clone dependency".red().bold()),
            Ext => write!(f, "An internal error occured. please try again."),
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
}

#[derive(Deserialize, Debug)]
struct DummyRoot {
    project: Project,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Remote(HashMap<String, String>);

#[derive(Deserialize, Debug, Clone)]
pub struct Dependencies {
    pub vcs: Option<String>,
    pub vcs_flags: Option<Vec<String>>,
    pub local: Option<Vec<String>>,
    pub remote: Option<Remote>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Build {
    pub compiler: String,
    pub sources: Vec<String>,
    pub includes: Option<Vec<String>>,
    pub warnings: Option<u8>,
    pub objects: Option<bool>,
    pub bin: Option<String>, // The name of the output
    pub dependencies: Option<Dependencies>,
}

// TODO: make this work properly, it's really hacky rn and should work recursively.
// NINE indentation layers... i definitely need to break this down into smaller functions!
fn get_sources<'a>(sources: &Vec<String>) -> Result<Vec<String>> {
    let mut out: Vec<String> = vec![];
    for source in sources.iter() {
        let source_path = fs::canonicalize(&source)?;
        if source_path.is_file() {
            out.push(source.clone());
        }
        if source_path.is_dir() {
            let dir = fs::read_dir(&source_path)?;
            for path in dir {
                let file = path?;
                let file = file.path();
                let extension = get_extension(&file)?;
                match extension {
                    "c" | "cpp" | "cxx" | "c++" | "cc" | "C" => {
                        if let Some(f) = file.to_str() {
                            out.push(f.to_string())
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(out)
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
    fn run(&self, command: &Command) -> Result<()> {
        match command {
            Command::build => self.build_project()?,
            _ => todo!("Only supported command is build"),
        }
        Ok(())
    }

    fn build_project(&self) -> Result<()> {
        println!(
            "{} {} v{}",
            "  Building".green().bold(),
            &self.project.name,
            &self.project.version
        );
        let build_sources = get_sources(&self.build.sources)?;
        if self.build.objects.unwrap_or(false) {
            let objects = self.compilation_stage(&build_sources)?;
            self.link_from(&objects)?;
        } else {
            self.link_from(&build_sources)?;
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
        let platform_links = self.get_local_links()?;
        self.load_remote_dependencies()?;
        if platform_links.is_some() {
            compiler.args(&platform_links.unwrap());
        }
        compiler.arg("-o").arg(&self.get_output_name());
        let status = compiler.status()?;
        if !status.success() {
            return Err(CompilationError::Linking(self.project.name.to_owned()).into());
        }
        Ok(())
    }

    fn get_local_links(&self) -> Result<Option<Vec<String>>> {
        if self.build.dependencies.is_none() {
            return Ok(None);
        }
        let deps = self.build.dependencies.clone().unwrap();
        let mut out: Vec<String> = vec![];
        // deps.local.clone().and_then(|deps| {
        if let Some(deps) = deps.local.clone() {
            for dep in deps {
                if dep.starts_with("`") && dep.ends_with("`") {
                    let mut buf = String::from("");
                    for c in dep.chars() {
                        if c != '`' {
                            buf += &c.to_string();
                        }
                    }
                    let cmds: Vec<&str> = buf.split(" ").collect();
                    let mut expansion = process::Command::new(cmds[0]);
                    expansion.args(&cmds[1..]);
                    let command_out = expansion.output()?; // replace with own error
                    let output = String::from_utf8(command_out.stdout)?;
                    for opt in output.split_ascii_whitespace() {
                        out.push(opt.into());
                    }
                    continue;
                }
                out.push("-l".into());
                out.push(dep);
            }
        }
        Ok(Some(out))
    }

    fn load_remote_dependencies(&self) -> Result<()> {
        if let Some(dep_root) = &self.build.dependencies {
            if let Some(remote) = &dep_root.remote {
                for (k, v) in remote.to_owned().0.into_iter() {
                    let repository = dependency_link(&v);
                    let mut git = process::Command::new(&self.get_vcs());
                    git.args(["clone", &repository, &k]);
                    if let Some(vcs_flags) = &dep_root.vcs_flags {
                        git.args(vcs_flags);
                    }
                    // suppress git output
                    git.stderr(Stdio::null());
                    let status = git.status()?;
                    if !status.success() {
                        return Err(CompilationError::Cloning(repository).into());
                    }
                }
            }
        }
        Ok(())
    }

    fn get_vcs(&self) -> String {
        if let Some(dependencies) = &self.build.dependencies {
            let vcs = dependencies.vcs.clone();
            vcs.unwrap_or("git".into())
        } else {
            String::from("git")
        }
    }

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

    fn has_been_modified(&self, source: &PathBuf, object: &PathBuf) -> Result<bool> {
        let source_meta = fs::metadata(&source)?;
        let object_meta = fs::metadata(&object)?;
        Ok(object_meta.modified()? < source_meta.modified()?)
    }

    fn compilation_stage(&self, sources: &Vec<String>) -> Result<Vec<String>> {
        let mut out: Vec<String> = vec![];
        for file in sources.iter() {
            let mut compiler = std::process::Command::new(&self.build.compiler);
            let mut output = canonicalize(PathBuf::from(file))?;
            let input = output.clone();
            output.set_extension("o");
            let needs_to_be_compiled = self.has_been_modified(&input, &output).unwrap_or(true);
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
            if needs_to_be_compiled {
                println!("    {} {}", "Compiling".green().bold(), file);
                let status = compiler.status()?;
                let status = status.success();
                if !status {
                    return Err(CompilationError::Compiling(self.project.name.to_owned()).into());
                }
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
        let cmd = cmd.as_ref().unwrap_or(&Command::build);
        match self {
            Self::App(app) => app.run(cmd)?,
            Self::Lib(lib) => lib.run(cmd),
        }
        Ok(())
    }
}

fn dependency_link(name: &String) -> String {
    if name.starts_with("https://") || name.starts_with("git@") {
        return name.clone();
    }
    "https://github.com/".to_string() + name
}
