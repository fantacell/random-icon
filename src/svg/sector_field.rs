use svg::node::element::path::{self, Data};

use super::{FieldBorder, FieldShape};

pub enum SectorField {
    Inner,
    InnerMid,
    OuterMid,
    Outer
}

impl FieldShape for SectorField {
    fn field_border_path_data(self) -> FieldBorder {
        let path_data = match self {
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
                .elliptical_arc_to((33.324, 33.324, 0, 0, 0, 0, -33.324))
                .close()
            },
            SectorField::InnerMid => {
                Data::new()
                .move_to((0, -33.324))
                .elliptical_arc_to((33.324, 33.324, 0, 0, 1, 20.651, -26.155))
                .vertical_line_to(-14.232)
                .line_to((20.651, -14.232))
                .line_to((0, 26.155))
                .close()
            },
            SectorField::Inner => {
                Data::new()
                .move_to((0, -26.155))
                .line_to((10.325, -8.271))
                .line_to((0, -2.309))
                .close()
            }
        }
        ;

        FieldBorder { path_data }
    }
}