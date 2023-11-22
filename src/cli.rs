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
}

impl Lang {
    pub fn to_extension<'a>(&self) -> &'a str {
        match self {
            Self::c => "c",
            Self::cpp => "cpp",
        }
    }
}
