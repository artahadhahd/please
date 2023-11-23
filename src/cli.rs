<<<<<<< HEAD
use clap::Parser;

#[derive(Parser)]
#[command(author = "artahadhahd", about = "build system", long_about = None)]
pub struct Cli {
    #[arg(short, long, num_args(0..))]
    pub paths: Vec<String>,
}

impl Cli {
    pub fn get_result(&self) -> Vec<String> {
        if self.paths.is_empty() {
            vec![".".into()]
        } else {
            self.paths.to_owned()
=======
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    pub initial: Option<Command>,
    #[arg(long)]
    pub new: Option<String>,
    #[arg(short, long)]
    pub language: Option<Lang>,
}

#[allow(non_camel_case_types)]
#[derive(ValueEnum, Clone, Debug)]
pub enum Command {
    run,
    build,
    scripts,
}

#[allow(non_camel_case_types)]
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum Lang {
    #[default]
    c,
    cpp,
    #[clap(name = "c++")]
    cxx,
}

impl Lang {
    pub fn to_extension<'a>(&self) -> &'a str {
        match self {
            Self::c => "c",
            _ => "cpp",
>>>>>>> origin/v2
        }
    }
}
