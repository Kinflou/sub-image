// Standard Uses
use std::path::Path;

// Crate Uses
use crate::formats::sub::SubInfo;
use crate::formats::subs::GroupSubInfo;

// External Uses
use anyhow::{Result};
use image::DynamicImage;
use image::io::Reader;


#[derive(Default)]
pub struct UnifiedImage {
    pub groups: Vec<GroupSubInfo>,
    pub subs: Vec<SubInfo>,
    image: DynamicImage
}


impl UnifiedImage {
    pub fn from_file(path: &Path) -> Result<Self> {
        let image = Reader::open(path)?.decode()?;

        Ok(Self {
            groups: vec![],
            subs: vec![],
            image
        })
    }

    pub fn name(&self) -> String {
        "test".to_owned()
    }

    pub fn add_sub(&mut self, info: SubInfo) {
        self.subs.push(info);
    }

    pub fn add_subs_group(&mut self, group: GroupSubInfo) {
        self.groups.push(group);
    }

    pub fn slice_subs(self) -> Vec<UnifiedImage> {
        todo!()
    }
}

