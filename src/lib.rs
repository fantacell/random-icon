pub mod random;

pub struct Fields {
    sectors: [Sector; 6], //clockwise
    sector_dividers: [SectorDivider; 3], //clockwise,
    center_field: Field
}

#[derive(Clone, Copy)]
pub struct Sector([Field; 4]); //inner to outer

#[derive(Clone, Copy)]
pub struct SectorDivider([Field; 3]); //outer to inner

#[derive(Clone, Copy)]
pub enum Field {
    Empty,
    Filled
}

pub enum Symmetry {
    OneAxis,
    ThreeAxes,
    Point,
}
