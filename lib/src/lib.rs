//! A crate for creating a random round image in
//! the svg format. Images are always symmetrical, in
//! order to be more recognizable. It varies between
//! transparent and black areas.
//! 
//! # Warning
//! 
//! An icon can be completely empty (transparent)
//! or only showing tiny areas. If you want something
//! to always be visible, consider giving it a border
//! or background.

mod fields;
pub mod hashing;
mod generate;
mod svg;

use std::{
    ffi::OsStr,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf}
};

use fields::Fields;
use generate::from_hash::HashGen;

pub use hashing::hash;


/// Converts text to an icon (via hashing) and writes
/// it as svg data. The same text will always result
/// in the same icon.
/// 
/// # Examples
/// 
/// ```
/// use random_icon_lib::write_svg_from_hashed_string;
/// 
/// let my_string = "Lorem ipsum dolor sit amet...".to_string();
/// let mut file = std::fs::File::create("./tests/my_svg_file.svg").unwrap();
/// 
/// write_svg_from_hashed_string(my_string, file).unwrap();
/// ```
pub fn write_svg_from_hashed_string<T: Write>(
    string: String,
    target: T,
) -> Result<(), io::Error> {
    let hash = hash(string);
    let hash_gen = HashGen::new(hash);
    let fields = Fields::new(hash_gen);

    fields.write_as_svg(target)
}

/// Creates a random icon and writes it as svg data.
/// 
/// # Examples
/// 
/// ```
/// use random_icon_lib::write_random_svg;
/// 
/// let mut file = std::fs::File::create("./tests/my_svg_file.svg")
///     .unwrap()
/// ;
/// 
/// write_random_svg(file).unwrap();
#[cfg(feature = "rand")]
pub fn write_random_svg<T: Write>(
    target: T,
) -> Result<(), io::Error> {
    let fields = Fields::new(rand::rng());

    fields.write_as_svg(target)
}

/// Convenience function to create a new file at the
/// given path and write an icon from a hashed string
/// to it.
/// 
/// # Errors
/// 
/// Among other tings returns an error if a file
/// already exists at the given path
/// 
/// # Examples
/// 
/// ```
/// use random_icon_lib::create_svg_file_from_hashed_string;
/// 
/// let my_string = "Lorem ipsum dolor sit amet...".to_string();
/// let path_to_new_file = "./tests/new_svg_file.svg";
/// 
/// create_svg_file_from_hashed_string(
///     my_string,
///     path_to_new_file
/// ).unwrap();
/// ```
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

/// Convenience function to create a new file at the
/// given path and write a random icon to it.
/// 
/// # Errors
/// 
/// Among other tings returns an error if a file
/// already exists at the given path
/// 
/// # Examples
/// 
/// ```
/// use random_icon_lib::create_random_svg_file;
/// 
/// let path_to_new_file = "./tests/new_random_svg_file.svg";
/// 
/// create_random_svg_file(
///     path_to_new_file
/// ).unwrap();
/// ```
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