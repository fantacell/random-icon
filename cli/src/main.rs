use random_icon_lib::{save_random_icon, save_random_icon_from_hashed_string};

fn main() {
    if let Some(string) = std::env::args().nth(1) {
        save_random_icon_from_hashed_string(
            string,
            "./icons".into(),
            None
        );
    } else {
        save_random_icon("./icons".into(), None);
    }
}
