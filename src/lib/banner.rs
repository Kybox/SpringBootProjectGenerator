use crate::PROJECT_DIR;
use std::env;

pub fn display_banner(){

    let banner_file = PROJECT_DIR.get_file("banner.txt").unwrap();
    let banner_content = banner_file.contents_utf8().unwrap();
    let current_path = env::current_dir().expect("Error get current directory");
    let banner = banner_content.replace("#path", current_path.to_str().unwrap());

    println!("{}", banner);
}
