use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub fn get_file(root: PathBuf, file: String) -> Result<String, std::io::Error> {
    let file_path = root.join(file);
    println!("Reading {:?}...", file_path);
    read_to_string(file_path)
}
