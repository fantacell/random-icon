use std::path::Path;

use crate::fields::Fields;

pub mod center_field;
pub mod path;
pub mod sector_field;

impl Fields {
    pub fn save_as_svg_file<L: AsRef<Path>>(self, location: L) {
        let document = svg::Document::new()
            .set("viewBox", (-50, -50, 50, 50))
            .set("width", "100")
            .set("height", "100")
        ;
    }
}

pub trait FieldShape {
    fn field_border_path_data(self) -> FieldBorder;
}

pub struct FieldBorder {
    path_data: svg::node::element::path::Data
}

//pub enum SectorFieldOriginalOrientation {
//    Inner,
//    InnerMid,
//    OuterMid,
//    Outer
//}
//
//pub enum SectorFieldMirrored {
//    Inner,
//    InnerMid,
//    OuterMid,
//    Outer
//}
//
//pub enum SectorDividerField {
//    Inner,
//    Mid,
//    Outer
//}
//