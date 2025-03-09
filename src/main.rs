use random_icon::{
    fields::Fields,
    hash::hash,
    random::{FromHash, HashGen}
};

fn main() {
    let string = std::env::args().nth(1).unwrap_or_default();
    let hash = hash(string);

    let mut hash_gen = HashGen::new(hash);
    let fields = Fields::from_hash(&mut hash_gen);

    let target_directory = std::path::PathBuf::from("./icons");
    let target_file_name = hash.to_string();

    let mut target_path = target_directory;
    target_path.push(target_file_name);

    fields.save_svg_file(target_path);
}
