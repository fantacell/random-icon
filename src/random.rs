use crate::{Field, Sector, SectorDivider, Symmetry};

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