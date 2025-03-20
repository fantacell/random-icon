use rand::{
    distr::{Distribution, StandardUniform},
    Rng
};

use crate::fields::{Field, Fields, Sector, SectorDivider};

impl Distribution<Fields> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fields {
        Fields {
            sectors: rng.random(),
            sector_dividers: rng.random(),
            center_field: rng.random()
        }
    }
}

impl Distribution<Sector> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sector {
        Sector(rng.random())
    }
}

impl Distribution<SectorDivider> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SectorDivider {
        SectorDivider(rng.random())
    }
}

impl Distribution<Field> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Field {
        if rng.random_bool(0.5) {
            Field::Filled
        } else {
            Field::Empty
        }
    }
}