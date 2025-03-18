use crate::fields::{Field, Fields};

use super::{FieldShape::{self, *}, Orientation};

impl Fields {
    pub(super) fn active_field_shapes_with_rotation(&self) -> Vec<(FieldShape, u16, Orientation)> {
        let mut vec = Vec::new();

        for (num_sector, sector) in self.sectors.iter().enumerate() {
            let sector_mirrored = num_sector % 2 == 1;
            let orientation = if sector_mirrored {
                Orientation::Mirrored
            } else {
                Orientation::Original
            };

            for (num_field, field) in sector.0.iter().enumerate() {
                if *field == Field::Empty {
                    continue
                }

                let field = match num_field {
                    0 => SectorInner,
                    1 => SectorInnerMid,
                    2 => SectorOuterMid,
                    3 => SectorOuter,
                    _ => panic!("unknown field number")
                };

                let angle = 60 * u16::try_from(num_sector).unwrap() + {
                    if sector_mirrored { 60 } else { 0 }
                };

                vec.push((field, angle, orientation));
            }
        }

        for (num_sector_divider, sector_divider) in self.sector_dividers.iter().enumerate() {
            for (num_field, field) in sector_divider.0.iter().enumerate() {
                if *field == Field::Empty {
                    continue
                }

                let field = match num_field {
                    0 => SectorDividerInner,
                    1 => SectorDividerMid,
                    2 => SectorDividerOuter,
                    _ => panic!("unknown field number")
                };

                let angle = 120 * u16::try_from(num_sector_divider).unwrap();

                vec.push((field, angle, Orientation::Original));
            }
        }

        if self.center_field == Field::Filled {
            vec.push((CenterField, 0, Orientation::Original));
        }
        ;

        vec
    }
}