use std::{fmt, path::PathBuf};
use colored::Colorize;

#[derive(Debug)]
pub enum CompilationError {
    Compiling(String),
    Linking(String),
    #[allow(dead_code)]
    Dependency(String),
    Cloning(String),
    ShellCommand(String),
    FailedToCreateFile(PathBuf),
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
            ShellCommand(command) => write!(f, "{} '{command}'\nMake sure it's installed on your machine", "Failed to run".red().bold()),
            FailedToCreateFile(file_name) => write!(f, "{} {:?}", "Couldn't create file".red().bold(), file_name),
            Ext => write!(f, "An internal error occured. please try again."),
        }
    }
}