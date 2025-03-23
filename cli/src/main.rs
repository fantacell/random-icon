use std::{io, path::PathBuf};

use clap::{command, value_parser, Arg, ValueHint};
use random_icon::to_file_name;
use random_icon_lib::{
    create_random_svg_file,
    create_svg_file_from_hashed_string
};

fn main() -> io::Result<()> {
    let matches = command!()
    .arg(
        Arg::new("input-string")
        .short('s')
        .long("string")
        .help("Generate the icon from a hashed string")
    )
    .arg(
        Arg::new("full-path")
        .short('p')
        .long("path")
        .value_hint(ValueHint::FilePath)
        .value_parser(value_parser!(PathBuf))
        //.value_name("FILE")
        .help("The path to the target file")
    )
    .arg(
        Arg::new("directory")
        .short('l')
        .long("location")
        .conflicts_with("full-path")
        .value_hint(ValueHint::DirPath)
        .value_parser(value_parser!(PathBuf))
        .help("The directory of the target file")
    )
    .arg(
        Arg::new("file-name")
        .short('f')
        .long("file-name")
        .conflicts_with("full-path")
        .value_parser(value_parser!(PathBuf))
        .help("The name of the target file")
    )
    .get_matches()
    ;

    let input_string = matches.get_one::<String>("input-string");

    let path = {
        if let Some(path) = matches.get_one::<PathBuf>("full-path") {
            path.clone()
        } else {
            let directory = matches
                .get_one::<PathBuf>("directory")
                .unwrap_or(&"./".into())
                .clone()
            ;
            let file_name = matches
                .get_one::<PathBuf>("file-name")
                .cloned()
                .unwrap_or(if let Some(input_string) = input_string {
                        to_file_name(input_string.to_owned())
                    } else {
                        "random_icon".into()
                    }
                )
                .clone()
            ;

            directory.join(file_name)
        }
    };

    if let Some(input_string) = input_string {
        create_svg_file_from_hashed_string(
            input_string.to_owned(),
            path
        )?
    } else {
        create_random_svg_file(
            path
        )?
    }

    Ok(())
}
