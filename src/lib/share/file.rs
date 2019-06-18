use crate::PROJECT_DIR;

pub fn get_file_content(file: &str) -> String {

    let tmp_file = PROJECT_DIR.get_file(file).unwrap();
    return String::from(tmp_file.contents_utf8().unwrap());
}
