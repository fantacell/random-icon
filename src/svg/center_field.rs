use svg::node::element::{path::{self, Data}, Path};

use super::Field;

pub struct CenterField;

impl Field for CenterField {
    fn svg_path_data(self) -> path::Data {
        Data::new()
            .move_to((50., 47.691))
            .line_to((52., 46.536))
            .line_to((54., 50.))
            .line_to((52., 51.155))
            .line_to((52., 53.464))
            .line_to((48., 53.464))
            .line_to((48., 51.155))
            .line_to((46., 50.))
            .line_to((48., 46.536))
            .close()
    }
}