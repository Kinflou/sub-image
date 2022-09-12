// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use anyhow::{bail, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct SubInfo {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32
}

impl SubInfo {
    pub fn from_path(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path).expect("Failed to load file");
        let extension = path.extension().unwrap().to_str().unwrap();

        let sub = match extension {
            "json5" => { from_json5(contents) }
            "sub" => { from_sub(contents) }
            _ => { bail!("No extension found for {}", extension)}
        }.expect("TODO: panic message");

        Ok(sub)
    }

}


pub fn from_json5(contents: String) -> Result<SubInfo> {
    let sub = json5::from_str::<SubInfo>(contents.as_str())?;

    Ok(sub)
}

pub fn from_sub(_contents: String) -> Result<SubInfo> {
    todo!()
}

