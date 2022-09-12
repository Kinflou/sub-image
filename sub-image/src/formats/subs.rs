use std::collections::HashMap;
// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses
use crate::formats::sub::SubInfo;

// External Uses
use anyhow::{bail, Result};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct GroupSubInfo {
    image: String,
    size: [u32; 2],
    pub(crate) subs: HashMap<String, SubInfo>
}

impl GroupSubInfo {
    pub fn from_path(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path).expect("Failed to load file");
        let extension = path.extension().unwrap().to_str().unwrap();

        let sub = match extension {
            "json5" => { from_json5(contents) }
            _ => { bail!("No extension found for {}", extension)}
        }.expect("Could not parse subs group");

        Ok(sub)
    }

}


pub fn from_json5(contents: String) -> Result<GroupSubInfo> {
    let sub = json5::from_str::<GroupSubInfo>(contents.as_str())?;

    Ok(sub)
}

