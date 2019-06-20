use crate::PROJECT_DIR;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn get_file_content(file: &str) -> String {

    let tmp_file = PROJECT_DIR.get_file(file).unwrap();
    return String::from(tmp_file.contents_utf8().unwrap());
}

pub fn write_file(path: &PathBuf, content: &str) {

    // Create the file
    let mut file = File::create(&path)
        .expect("/!\\ Can't create the the file");

    // Dump the processed contents to it
    file.write(content.as_bytes())
        .expect("/!\\ Can't write content to the file");
}
