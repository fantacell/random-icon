use std::path::PathBuf;

pub fn to_file_name(string: String) -> PathBuf {
    const FILE_NAME_LENGTH: usize = 15;

    let file_name: String = string
        .chars()
        .take(FILE_NAME_LENGTH)
        .collect()
    ;

    file_name.into()
}