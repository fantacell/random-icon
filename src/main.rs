use random_icon::{hash::hash, Fields};

fn main() {
    let string = std::env::args().nth(1).unwrap_or_default();
    let hash = hash(string);
}
