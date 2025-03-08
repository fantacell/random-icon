use svg::node::element::{path::{self, Data}, Path};

use super::FieldShape;

pub struct CenterField;

impl FieldShape for CenterField {
    fn svg_shape_data(self) -> path::Data {
        Data::new()
            .move_to((50. - 50., 47.691 - 50.))
            .line_to((52. - 50., 46.536 - 50.))
            .line_to((54. - 50., 50. - 50.))
            .line_to((52. - 50., 51.155 - 50.))
            .line_to((52. - 50., 53.464 - 50.))
            .line_to((48. - 50., 53.464 - 50.))
            .line_to((48. - 50., 51.155 - 50.))
            .line_to((46. - 50., 50. - 50.))
            .line_to((48. - 50., 46.536 - 50.))
            .close()
    }
}