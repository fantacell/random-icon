pub struct Fields {
    pub sectors: [Sector; 6], //clockwise
    pub sector_dividers: [SectorDivider; 3], //clockwise,
    pub center_field: Field
}

#[derive(Clone, Copy)]
pub struct Sector(pub [Field; 4]); //inner to outer

#[derive(Clone, Copy)]
pub struct SectorDivider(pub [Field; 3]); //inner to outer

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
