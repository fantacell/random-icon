use crate::fields::{Field, Symmetry};

use std::ops::Range;

use super::FieldsGen;

#[derive(Clone, Copy)]
pub struct HashGen {
    hash: u32
}

impl HashGen {
    pub fn new(hash: u32) -> Self {
        Self { hash }
    }

    pub fn gen_range_cycle(&mut self, range: Range<u32>) -> u32 {
        let available_bits = u32::MIN.count_zeros();
        let required_bits = available_bits - (range.end - 1).leading_zeros();

        let template: u32 = (1 << required_bits) - 1;

        loop {
            let generated_value = self.hash & template;

            self.hash = self.hash.rotate_left(required_bits);

            if range.contains(&generated_value) {
                break generated_value;
            }
        }
    }

     pub fn gen_bool_cycle(&mut self) -> bool {
        self.gen_range_cycle(0..2) != 0
    }
}

impl FieldsGen for HashGen {
    fn symmetry(&mut self) -> Symmetry {
        match self.gen_range_cycle(0..3) {
            0 => Symmetry::OneAxis,
            1 => Symmetry::ThreeAxes,
            2 => Symmetry::Point,
            _ => panic!("generated unfitting value")
        }
    }

    fn field(&mut self) -> Field {
        if self.gen_bool_cycle() {
            Field::Filled
        } else {
            Field::Empty
        }
    }
}