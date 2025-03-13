use svg::node::element::path::Data;

use super::{FieldBorder, FieldShape::{self, *}};

impl FieldShape {
    pub fn field_border_path_data(self) -> FieldBorder {
        let path_data = match self {
            SectorOuter => {
                Data::new()
                .move_to((0, -50))
                .elliptical_arc_to((50, 50, 0, 0, 1, 42.267, -26.712))
                .line_to((27.808, -18.364))
                .elliptical_arc_to((33.324, 33.324, 0, 0, 0, 20.651, -26.155))
                .vertical_line_to(-38.077)
                .close()
            },
            SectorOuterMid => {
                Data::new()
                .move_to((0, -50))
                .line_to((20.651, -38.077))
                .vertical_line_to(-26.155)
                .elliptical_arc_to((33.324, 33.324, 0, 0, 0, 0, -33.324))
                .close()
            },
            SectorInnerMid => {
                Data::new()
                .move_to((0, -33.324))
                .elliptical_arc_to((33.324, 33.324, 0, 0, 1, 20.651, -26.155))
                .vertical_line_to(-14.232)
                .line_to((20.651, -14.232))
                .line_to((10.325, -8.271))
                .line_to((0, -26.155))
                .close()
            },
            SectorInner => {
                Data::new()
                .move_to((0, -26.155))
                .line_to((10.325, -8.271))
                .line_to((0, -2.309))
                .close()
            },
            SectorMirroredOuter => {
                SectorOuter.field_border_path_data().path_data
            },
            SectorMirroredOuterMid => {
                SectorOuterMid.field_border_path_data().path_data
            },
            SectorMirroredInnerMid => {
                SectorInnerMid.field_border_path_data().path_data
            },
            SectorMirroredInner => {
                SectorMirroredInner.field_border_path_data().path_data
            },
            SectorDividerInner => {
                Data::new()
                .move_to((0, 0))
                .vertical_line_to(-2.309)
                .line_to((10.325, -8.271))
                .line_to((12.325, -4.807))
                .line_to((2.00, 1.155))
                .close()
            },
            SectorDividerMid => {
                Data::new()
                .move_to((10.325, -8.271))
                .line_to((20.651, -14.232))
                .vertical_line_to(-26.155)
                .elliptical_arc_to((33.324, 33.324, 0, 0, 1, 32.976, -4.807))
                .line_to((22.651, -10.768))
                .line_to((12.325, -4.807))
                .close()
            },
            SectorDividerOuter => {
                Data::new()
                .move_to((42.267, -26.712))
                .elliptical_arc_to((50, 50, 0, 0, 1, 44.267, -23.248))
                .line_to((29.808, -14.90))
                .elliptical_arc_to((50, 50, 0, 0, 0, 27.808, -18.364))
                .close()
            },
            CenterField => {
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
        };

        FieldBorder { path_data }
    }
}