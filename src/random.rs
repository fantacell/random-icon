use crate::{Field, Fields, Sector, SectorDivider, Symmetry};

use std::ops::Range;

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

            self.hash <<= required_bits;

            if range.contains(&generated_value) {
                break generated_value;
            }
        }
    }

     pub fn gen_bool_cycle(&mut self) -> bool {
        self.gen_range_cycle(0..2) != 0
    }
}

pub trait FromHash {
    fn from_hash(hash_gen: &mut HashGen) -> Self;
}

impl FromHash for Fields {
    fn from_hash(hash_gen: &mut HashGen) -> Self {
        let symmetry = Symmetry::from_hash(hash_gen);
        let center_field: Field = Field::from_hash(hash_gen);

        match symmetry {
            Symmetry::OneAxis => {
                let sectors = [Sector::from_hash(hash_gen); 3];
                let sector_dividers = [SectorDivider::from_hash(hash_gen); 2];

                let all_sectors = [sectors[0], sectors[1], sectors[2], sectors[2], sectors[1], sectors[0]];
                let all_sector_dividers = [sector_dividers[0], sector_dividers[1], sector_dividers[0]];

                Self {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            },
            Symmetry::ThreeAxes => {
                let sector = Sector::from_hash(hash_gen);
                let sector_divider = SectorDivider::from_hash(hash_gen);

                let all_sectors = [sector; 6];
                let all_sector_dividers = [sector_divider; 3];

                Self {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            },
            Symmetry::Point => {
                let sectors = [Sector::from_hash(hash_gen); 2];
                let sector_divider = SectorDivider::from_hash(hash_gen);

                let all_sectors = [sectors[0], sectors[1], sectors[0], sectors[1], sectors[0], sectors[1]];
                let all_sector_dividers = [sector_divider; 3];

                Self {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            }
        }
    }
}

impl FromHash for Symmetry {
    fn from_hash(hash_gen: &mut HashGen) -> Self {
        match hash_gen.gen_range_cycle(0..3) {
            0 => Symmetry::OneAxis,
            1 => Symmetry::ThreeAxes,
            2 => Symmetry::Point,
            _ => panic!("generated unfitting value")
        }
    }
}

impl FromHash for Sector {
    fn from_hash(hash_gen: &mut HashGen) -> Self {
        let mut sector = Self::default();

        for mut _field in &mut sector.0 {
            _field = &mut Field::from_hash(hash_gen);
        }

        sector
    }
}

impl FromHash for SectorDivider {
    fn from_hash(hash_gen: &mut HashGen) -> Self {
        let mut sector_divider = Self::default();

        for mut _field in &mut sector_divider.0 {
            _field = &mut Field::from_hash(hash_gen);
        }

        sector_divider
    }
}

impl FromHash for Field {
    fn from_hash(hash_gen: &mut HashGen) -> Self {
        if hash_gen.gen_bool_cycle() {
            Self::Filled
        } else {
            Self::Empty
        }
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

impl Default for Field {
    fn default() -> Self {
        Self::Empty
    }
}