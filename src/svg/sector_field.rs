use svg::node::element::path::{self, Data};

use super::Field;

pub struct CenterField;

impl Field for CenterField {
    fn svg_path_data(self) -> path::Data {
        Data::new()
            .move_by((20.652, 11.923))
            .vertical_line_by(11.932)
            .elliptical_arc_by((33.324, 33.324, 0, 0, 0, -20.651, -7.17))
            .close()
    }
}