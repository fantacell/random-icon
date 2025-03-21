mod fields;
pub mod hash;
mod generate;
mod svg;

use std::{
    ffi::OsStr,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf}
};

use fields::Fields;
use hash::hash;
use generate::from_hash::HashGen;

pub fn write_svg_from_hashed_string<T: Write>(
    string: String,
    target: T,
) -> Result<(), io::Error> {
    let hash = hash(string);
    let hash_gen = HashGen::new(hash);
    let fields = Fields::new(hash_gen);

    fields.write_as_svg(target)
}

#[cfg(feature = "rand")]
pub fn write_random_svg<T: Write>(
    target: T,
) -> Result<(), io::Error> {
    let fields = Fields::new(rand::rng());

    fields.write_as_svg(target)
}

pub fn create_svg_file_from_hashed_string<P: AsRef<Path>>(
    string: String,
    path: P
) -> io::Result<()> {
    let target = create_svg_file(
        path.as_ref().into()
    )?;

    write_svg_from_hashed_string(string, target)?;

    Ok(())
}

#[cfg(feature = "rand")]
pub fn create_random_svg_file<P: AsRef<Path>>(
    path: P
) -> io::Result<()> {
    let target = create_svg_file(
        path.as_ref().into()
    )?;

    write_random_svg(target)?;

    Ok(())
}

fn create_svg_file(
    mut path: PathBuf
) -> io::Result<File> {
    if path.extension() != Some(OsStr::new("svg")) {
        path.set_extension("svg");
    };

    File::create_new(path)
}