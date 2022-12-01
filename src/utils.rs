use crate::result::*;

pub fn read_input_file(input_file: &str) -> Result<String> {
    use std::{fs, path};

    let filepath = path::PathBuf::from("./resources/").join(input_file);

    let err_str = format!("File not found: {:?}", filepath);

    return fs::read_to_string(filepath).with_context(|| err_str);
}

