pub mod random;
pub mod svg;

use random::Random;

pub struct Fields {
    sectors: [Sector; 6], //clockwise
    sector_dividers: [SectorDivider; 3], //clockwise,
    center_field: FieldValue
}

impl Random for Fields {
    fn random() -> Self {
        let rand_symmetry = Symmetry::random();

        match rand_symmetry {
            Symmetry::OneAxis => {
                let sectors = [Sector::random(); 3];
                let sector_dividers = [SectorDivider::random(); 2];
                let center_field = FieldValue::random();

                let all_sectors = [sectors[0], sectors[1], sectors[2], sectors[2], sectors[1], sectors[0]];
                let all_sector_dividers = [sector_dividers[0], sector_dividers[1], sector_dividers[0]];

                Self {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            },
            Symmetry::ThreeAxes => {
                let sector = Sector::random();
                let sector_divider = SectorDivider::random();
                let center_field = FieldValue::random();

                let all_sectors = [sector; 6];
                let all_sector_dividers = [sector_divider; 3];

                Self {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            },
            Symmetry::Point => {
                let sectors = [Sector::random(); 2];
                let sector_divider = SectorDivider::random();
                let center_field = FieldValue::random();

                let all_sectors = [[sectors[0], sectors[1]]; 3].as_flattened().try_into().unwrap();
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

#[derive(Clone, Copy)]
pub struct Sector([FieldValue; 4]); //inner to outer

#[derive(Clone, Copy)]
pub struct SectorDivider([FieldValue; 3]); //outer to inner

#[derive(Clone, Copy)]
pub enum FieldValue {
    Empty,
    Filled
}

pub enum Symmetry {
    OneAxis,
    ThreeAxes,
    Point,
}
