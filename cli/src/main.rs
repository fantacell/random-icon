use random_icon_lib::save_random_icon_from_hashed_string;

fn main() {
    let string = std::env::args()
        .nth(1)
        .unwrap_or("default".to_string())
    ;

    save_random_icon_from_hashed_string(
        string,
        "./icons".into(),
        None
    );
}
