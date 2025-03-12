use crate::fields::{Field, Fields};

use super::FieldShape::{self, *};

impl Fields {
    pub(super) fn active_field_shapes_with_rotation(&self) -> Vec<(FieldShape, u16)> {
        let mut vec = Vec::new();

        for (num_sector, sector) in self.sectors.iter().enumerate() {
            let sector_mirrored = num_sector % 2 == 1;

            for (num_field, field) in sector.0.iter().enumerate() {
                if *field == Field::Empty {
                    continue
                }

                let field = if sector_mirrored { match num_field {
                    0 => SectorInner,
                    1 => SectorInnerMid,
                    2 => SectorOuterMid,
                    3 => SectorOuter,
                    _ => panic!("unknown field number")
                }} else { match num_field {
                    0 => SectorMirroredInner,
                    1 => SectorMirroredInnerMid,
                    2 => SectorMirroredOuterMid,
                    3 => SectorMirroredOuter,
                    _ => panic!("unknown field number")
                }};

                let angle = 120 * u16::try_from(num_sector).unwrap();

                vec.push((field, angle));
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

                vec.push((field, angle));
            }
        }

        if self.center_field == Field::Filled {
            vec.push((CenterField, 0));
        }
        ;

        vec
    }
}