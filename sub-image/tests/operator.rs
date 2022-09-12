// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use sub_image::formats::image::UnifiedImage;
use sub_image::operator::sub::Operator;
use sub_image::formats::subs::GroupSubInfo;


const UNIFIED_IMAGE_PATH: &str = "tests/data/unified_image.png";
const SUB_JSON_PATH: &str = "tests/data/json/info.json5";


#[test]
fn create_operator() {
    let mut operator = Operator::default();

    let mut image = UnifiedImage::from_file(Path::new(UNIFIED_IMAGE_PATH))
        .unwrap();

    let path = Path::new(SUB_JSON_PATH);

    let subs = GroupSubInfo::from_path(path).unwrap();
    image.add_subs_group(subs);

    operator.add_image(image);
}
