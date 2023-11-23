use anyhow::Result;
use std::path::PathBuf;

use crate::parse::CompilationError;

pub fn get_extension<'a>(file: &'a PathBuf) -> Result<&'a str> {
    if let Some(extension) = file.extension() {
        if let Some(extension) = extension.to_str() {
            return Ok(extension);
        }
    }
    Err(CompilationError::Ext.into())
}
