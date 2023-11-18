use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Language {
    Language(String),
    Languages(Vec<String>),
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
pub enum ProjectType {
    app,
    lib,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub r#type: ProjectType,
    pub language: Option<Language>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct StaticBuild {
    pub script: Option<String>,
    pub from: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Build {
    pub source: Vec<String>,
    pub include: Vec<String>,
    pub compiler: String,
    pub objects: Option<bool>,
    pub r#static: Option<StaticBuild>,
    // pub output: Option<bool>,
    // pub release: Option<ReleaseBuild>,
    // pub debug: Option<DebugBuild>,
    // pub warning: Option<u8>,
    // pub optimize: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct Dependency {
    pub r#static: Option<HashMap<String, String>>,
    pub shared: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct BuildConfiguration {
    pub project: Project,
    pub build: Build,
    pub dependencies: Option<Dependency>,
}
