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

    let mut target_file = std::path::PathBuf::from("./icons");
    target_file.push(hash.to_string());
}
