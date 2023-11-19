use clap::{Parser, ValueEnum};

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

#[derive(Parser, Debug)]
pub struct Cli {
    pub initial: Option<Command>,
    #[arg(long)]
    pub new: Option<String>,
    #[arg(short, long)]
    pub language: Option<Lang>,
}
