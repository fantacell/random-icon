#[derive(Debug)]
pub struct Fields {
    pub sectors: [Sector; 6],                //clockwise
    pub sector_dividers: [SectorDivider; 3], //clockwise,
    pub center_field: Field,
}

#[derive(Clone, Debug, Copy)]
pub struct Sector(pub [Field; 4]); //inner to outer

#[derive(Debug, Clone, Copy)]
pub struct SectorDivider(pub [Field; 3]); //inner to outer

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Empty,
    Filled,
}

#[derive(Debug)]
pub enum Symmetry {
    OneAxis,
    ThreeAxes,
    Point,
}
