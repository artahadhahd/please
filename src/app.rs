// use std::process::Command;

use crate::BuildSystem;
use anyhow::Result;

pub struct App {
    pub builds: Vec<BuildSystem>,
}

impl App {
    pub fn new(paths: Vec<String>) -> Result<Self> {
        let mut builds: Vec<BuildSystem> = vec![];
        for path in paths {
            builds.push(BuildSystem::new(&path)?);
        }
        Ok(Self { builds })
    }

    pub fn run(&self) -> Result<()> {
        for build in self.builds.iter() {
            build.construct().expect("Failed");
        }
        Ok(())
    }
}
