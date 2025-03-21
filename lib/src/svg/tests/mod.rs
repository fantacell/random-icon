
use std::{fs::File, path::{Path, PathBuf}};
use crate::fields::{Field, Fields, Sector};

#[test]
fn sector_fields() {
    show_all_single_sector_fields(0, "original".into());
}

#[test]
fn sector_fields_mirrored() {
    show_all_single_sector_fields(1, "mirrored".into());
}

#[test]
fn sector_fields_rotated() {
    show_all_single_sector_fields(2, "rotated".into());
}

fn show_all_single_sector_fields(sector_number: usize, file_name_start: PathBuf) {
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
        let path = format!(
            "./src/svg/tests/test_files/{}_field{}",
            file_name_start.to_str().unwrap(),
            i
        );
        let path = Path::new(&path);
        let file = File::create(path).unwrap();
        fields.write_as_svg(
            file
        ).unwrap();
    }
}