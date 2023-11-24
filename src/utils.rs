use anyhow::Result;
use std::path::PathBuf;

use crate::errors::CompilationError;

pub fn get_extension<'a>(file: &'a PathBuf) -> Result<&'a str> {
    if let Some(extension) = file.extension() {
        if let Some(extension) = extension.to_str() {
            return Ok(extension);
        }
    }
    Err(CompilationError::Ext.into())
}
