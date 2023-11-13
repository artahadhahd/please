use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Language {
    Language(String),
    Languages(Vec<String>),
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub version: String,
    // pub language: Option<Language>,
    // pub authors: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ReleaseBuild {
    pub warning: Option<u8>,
    pub debug: Option<u8>,
    pub optimize: Option<u8>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct DebugBuild {
    pub warning: Option<bool>,
    pub debug: Option<u8>,
    pub optimize: Option<u8>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Build {
    pub source: Vec<String>,
    pub include: Vec<String>,
    pub compiler: String,
    // pub objects: Option<bool>,
    // pub output: Option<bool>,
    // pub release: Option<ReleaseBuild>,
    // pub debug: Option<DebugBuild>,
    // pub warning: Option<u8>,
    // pub optimize: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct BuildConfiguration {
    pub project: Project,
    pub build: Build,
    // pub dependencies: Option<toml::Table>,
}
