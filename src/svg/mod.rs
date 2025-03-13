pub mod index;
pub mod shape_data;

use crate::fields::Fields;

impl Fields {
    pub fn save_as_svg_file(self, location: std::path::PathBuf) {
        let mut document = svg::Document::new()
            .set("viewBox", (-50, -50, 100, 100))
            .set("width", "100")
            .set("height", "100")
        ;
        
        for (field_shape, degrees) in self.active_field_shapes_with_rotation() {
            let mirrored = match field_shape {
                FieldShape::SectorMirroredInner => true,
                FieldShape::SectorMirroredInnerMid => true,
                FieldShape::SectorMirroredOuterMid => true,
                FieldShape::SectorMirroredOuter => true,
                _ => false
            };
            let mut transform_str = format!("rotate({}, 0, 0)", degrees);
            if mirrored {
                transform_str.push_str(" scale(-1,1)");
            }
            let path_element = svg::node::element::Path::new()
                .set("fill", "black")
                .set("d", field_shape.field_border_path_data().path_data)
                .set("transform", transform_str)
            ;

            document = document.add(path_element)
        }

        svg::save(location.with_extension("svg"), &document).unwrap();
    }
}

pub enum FieldShape {
    SectorInner,
    SectorInnerMid,
    SectorOuterMid,
    SectorOuter,
    SectorMirroredInner,
    SectorMirroredInnerMid,
    SectorMirroredOuterMid,
    SectorMirroredOuter,
    SectorDividerInner,
    SectorDividerMid,
    SectorDividerOuter,
    CenterField
}

pub struct FieldBorder {
    path_data: svg::node::element::path::Data
}
