use svg::node::element::path::{self, Data};

use super::FieldShape;

pub enum SectorField {
    Inner,
    InnerMid,
    OuterMid,
    Outer
}

impl FieldShape for SectorField {
    fn svg_shape_data(self) -> path::Data {
        match self {
            SectorField::Outer => {
                Data::new()
                .move_to((0, -50))
                .elliptical_arc_to((50, 50, 0, 0, 1, 42.267, -26.712))
                .line_to((27.808, -18.364))
                .elliptical_arc_to((33.324, 33.324, 0, 0, 0, 27.808, -18.364))
                .vertical_line_to(-38.077)
                .close()
            },
            SectorField::OuterMid => {
                Data::new()
                .move_to((0, -50))
                .line_to((20.651, -38.077))
                .vertical_line_to(-26.155)
                .elliptical_arc_to((33.324, 33.324, 0, 0, 0, 0, 33.324))
                .close()
            }
        }
    }
}