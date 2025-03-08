pub mod center_field;
pub mod rotate;
pub mod sector_field;

use svg::{node::element::path, Document};

use crate::Fields;

impl From<Fields> for Document {
    fn from(value: Fields) -> Self {
        
    }
}

pub trait FieldShape {
    fn svg_shape_data(self) -> path::Data;
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