use crate::fields::{FieldGroup, Fields, Symmetry};

pub mod from_hash;
#[cfg(feature = "rand")]
pub mod random;

pub trait FieldsGen<T: FieldGroup> {
    fn symmetry(&mut self) -> Symmetry;

    fn field_group(&mut self) -> T;
}

impl<G: FieldsGen<Fields>> Fields {
    pub fn new(&mut gen: G) -> Self {

    }
}