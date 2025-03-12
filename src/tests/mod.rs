#[cfg(test)]
pub mod tests {
    use crate::fields::{Field, Fields, Sector};

    #[test]
    fn sector_fields() {
        show_all_single_sector_fields(0);
    }
    
    #[test]
    fn sector_fields_rotated() {
        show_all_single_sector_fields(1);
    }

    fn show_all_single_sector_fields(sector_number: usize) {
        let mut sectors: [Sector; 4] = Default::default();
        for (i, sector) in sectors.iter_mut().enumerate() {
            sector.0[i] = Field::Filled;
        }
    
        let mut fields_instances = Vec::new();
    
        for sector in sectors {
            let mut fields_sectors: [Sector; 6] = Default::default();
            fields_sectors[sector_number] = sector;
    
            let fields = Fields {
                sectors: fields_sectors,
                sector_dividers: Default::default(),
                center_field: Default::default()
            };
            fields_instances.push(fields);
        }

        for (i, fields) in fields_instances.into_iter().enumerate() {
            fields.save_as_svg_file(
                format!("./src/tests/test_files/field{}", i).into()
            );
        }
    }
}