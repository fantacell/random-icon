use std::path::Path;

use crate::fields::Fields;

pub mod center_field;
pub mod path;
pub mod sector_field;

impl Fields {
    pub fn save_as_svg_file(self, location: std::path::PathBuf) {
        let mut document = svg::Document::new()
            .set("viewBox", (-50, -50, 50, 50))
            .set("width", "100")
            .set("height", "100")
        ;
        
        for (field_shape, degrees) in self.active_field_shapes_with_rotation() {
            let path_element = svg::node::element::Path::new()
                .set("fill", "black")
                .set("d", field_shape)
                .set("transform", format!("rotate({}, 0, 0)", degrees))
            ;

            document = document.add(path_element)
        }

        svg::save(location, &document).unwrap();
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