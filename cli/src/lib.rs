use std::path::PathBuf;

pub fn to_file_name(string: String) -> PathBuf {
    const FILE_NAME_LENGTH: usize = 15;

    let mut file_name = String::new();
    for word in string.split_whitespace() {
        if file_name.len() < FILE_NAME_LENGTH {
            file_name.push('_');
        }
        for char in word.chars() {
            if file_name.len() < FILE_NAME_LENGTH {
                file_name.push(char);
            }
        }
    }

    file_name.into()
}