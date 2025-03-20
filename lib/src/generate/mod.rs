use core::array;

use crate::fields::{Field, Fields, Sector, SectorDivider, Symmetry};

pub mod from_hash;
#[cfg(feature = "rand")]
pub mod random;

impl Fields {
    pub fn new<G: FieldsGen>(mut gen: G) -> Self {
        gen.fields()
    }
}

pub trait FieldsGen {
    fn symmetry(&mut self) -> Symmetry;

    fn field(&mut self) -> Field;

    fn sector(&mut self) -> Sector {
        Sector(array::from_fn(|_| self.field()))
    }
    fn sector_divider(&mut self) -> SectorDivider {
        SectorDivider(array::from_fn(|_| self.field()))
    }

    fn fields(&mut self) -> Fields {
        let symmetry = self.symmetry();
        let center_field: Field = self.field();

        match symmetry {
            Symmetry::OneAxis => {
                let sectors: [_; 3] = array::from_fn(|_| self.sector());
                let sector_dividers: [_; 3] = array::from_fn(|_| self.sector_divider());

                let all_sectors = [sectors[0], sectors[1], sectors[2], sectors[2], sectors[1], sectors[0]];
                let all_sector_dividers = [sector_dividers[0], sector_dividers[1], sector_dividers[0]];

                Fields {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            },
            Symmetry::ThreeAxes => {
                let sector = self.sector();
                let sector_divider = self.sector_divider();

                let all_sectors = [sector; 6];
                let all_sector_dividers = [sector_divider; 3];

                Fields {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            },
            Symmetry::Point => {
                let sectors: [_; 3] = array::from_fn(|_| self.sector());
                let sector_divider = self.sector_divider();

                let all_sectors = [sectors[0], sectors[1], sectors[0], sectors[1], sectors[0], sectors[1]];
                let all_sector_dividers = [sector_divider; 3];

                Fields {
                    sectors: all_sectors,
                    sector_dividers: all_sector_dividers,
                    center_field
                }
            }
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