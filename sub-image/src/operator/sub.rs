// Standard Uses

// Crate Uses
use crate::formats::image::UnifiedImage;

// External Uses


#[derive(Default)]
pub struct Operator {
    pub images: Vec<UnifiedImage>
}

impl Operator {
    pub fn add_image(&mut self, image: UnifiedImage) {
        self.images.push(image)
    }
}
