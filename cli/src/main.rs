use std::path::PathBuf;

use clap::{arg, command, value_parser, Arg};
use random_icon_lib::{
    save_random_icon,
    save_random_icon_from_hashed_string
};

fn main() {
    let matches = command!()
    .arg(
        Arg::new("input-string")
            .short('s')
            .long("string")
            .required(false)
            .help("Generate the icon from a hashed string")
    )
    .arg(
        arg!(
            -l --location <FILE> "The target location"
        )
        .value_parser(value_parser!(PathBuf))
        .required(true)
    )
    .get_matches()
    ;

    let location = matches.get_one::<PathBuf>("location")
        .unwrap()
        .clone()
    ;

    if let Some(string) = matches.get_one::<String>("input-string") {
        save_random_icon_from_hashed_string(string.clone(), location, None);
    } else {
        save_random_icon(location, None);
    }
}
