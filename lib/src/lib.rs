use std::path::PathBuf;

use fields::Fields;
use hash::hash;
use random::{FromHash, HashGen};

mod fields;
pub mod hash;
mod random;
mod svg;
mod tests;

pub fn save_random_icon_from_hashed_string(
    string: String,
    target_dir: PathBuf,
    filename: Option<String>
) {
    let hash = hash(string);

    let mut hash_gen = HashGen::new(hash);
    let fields = Fields::from_hash(&mut hash_gen);

    let filename = filename.unwrap_or(hash.to_string());
    let target_path = target_dir.join(filename);

    fields.save_as_svg_file(target_path);
}
