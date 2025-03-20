use rand::{Rng, rngs::ThreadRng};

use crate::fields::{Field, Symmetry};

use super::FieldsGen;

impl FieldsGen for ThreadRng {
    fn symmetry(&mut self) -> crate::fields::Symmetry {
        match self.random_range(0..3) {
            0 => Symmetry::OneAxis,
            1 => Symmetry::ThreeAxes,
            2 => Symmetry::Point,
            _ => panic!("generated unfitting value")
        }
    }

    fn field(&mut self) -> Field {
        if self.random_bool(0.5) {
            Field::Filled
        } else {
            Field::Empty
        }
    }
}
