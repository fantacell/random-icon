use crate::{Field, Fields, Sector, SectorDivider, Symmetry};

use std::ops::{Deref, Range, RangeBounds, RangeInclusive};

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

            self.hash <<= required_bits;

            if range.contains(&generated_value) {
                break generated_value;
            }
        }
    }
}

impl Fields {
    pub fn from_hash(hash: u32) -> Self {
        let hash_gen = HashGen::new(hash);

        let symmetry = match hash_gen.gen_range_cycle(0..3) {
            0 => Symmetry::OneAxis,
            1 => Symmetry::ThreeAxes,
            2 => Symmetry::Point,
            _ => panic!("generated unfitting value")
        };

        let 

        //let mut used_bits = 0;
//
        //let symmetry_number = if (0..3).contains(&(hash & 0b111111u32.rotate_right(6 + used_bits)).rotate_left(6 + used_bits)) {
        //    (hash & 0b111111u32.rotate_right(6 + used_bits)).rotate_left(6 + used_bits)
        //} else {
//
        //}
//
        //let mut sectors = [Sector::default(); 3];
        //for i in 0..3 {
        //    let four_bits = (hash & 0b1111u32.rotate_right((i + 1) * 4 + used_bits)).rotate_left((i + 1) * 4 + used_bits);
        //    used_bits += 4;
//
        //    sectors[i.try_into().unwrap()] = Sector::from_hash(four_bits.try_into().unwrap())
        //}
//
        //let mut sector_dividers = [SectorDivider::default(); 3];
        //for i in 0..2 {
        //    let three_bits = (hash & 0b111u32.rotate_right((i + 1) * 3 + used_bits)).rotate_left((i + 1) * 3 + used_bits);
        //    used_bits += 3;
//
        //    sectors[i.try_into().unwrap()] = Sector::from_hash(three_bits.try_into().unwrap())
        //}
//
        //let center_field = if ((hash & 1u32.rotate_right(1 + used_bits)).rotate_right(1 + used_bits)) == 0 {
        //    Field::Empty
        //} else {
        //    Field::Filled
        //};
        //used_bits += 1;
//
        //Self {
        //    sectors,
        //    sector_dividers,
        //    center_field
        //}
    }
}

impl From<HashGen> for Sector {
    fn from(mut value: HashGen) -> Self {
        let 
    }
}




impl Sector {
    fn from_hash(hash: u8) -> Self {
        let mut sector = Self::default();

        for i in 0..4usize {
            let bit = hash & (0b1000 >> i);
            let bool: bool = bit != 0;

            if bool {
                sector.0[i] = Field::Filled
            }
        }

        sector
    }
}

impl SectorDivider {
    fn from_hash(hash: u8) -> Self {
        let mut sector_divider = Self::default();

        for i in 0..3usize {
            let bit = hash & (0b100 >> i);
            let bool: bool = bit != 0;

            if bool {
                sector_divider.0[i] = Field::Filled
            }
        }

        sector_divider
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::Empty
    }
}

impl Default for Sector {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Default for SectorDivider {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub trait Random {
    fn random() -> Self;
}

impl Random for Field {
    fn random() -> Self {
        Self::Filled
    }
}

impl Random for Sector {
    fn random() -> Self {
        Self([Field::random(); 4])
    }
}

impl Random for SectorDivider {
    fn random() -> Self {
        Self([Field::random(); 3])
    }
}

impl Random for Symmetry {
    fn random() -> Self {
        Self::OneAxis
    }
}