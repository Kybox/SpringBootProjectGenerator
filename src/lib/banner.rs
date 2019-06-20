use std::env;

use colored::Colorize;

use crate::PROJECT_DIR;
use crate::lib::share::file::get_file_content;
use crate::lib::share::input::get_user_input;
use std::process::exit;

pub fn display_banner() {
    let banner_file = PROJECT_DIR.get_file("banner.txt").unwrap();
    let banner_content = banner_file.contents_utf8().unwrap();

    println!("{}", banner_content.bold().green());
}

pub fn display_header() {

    let header_file = PROJECT_DIR.get_file("header.txt").unwrap();
    let header_content = header_file.contents_utf8().unwrap();
    let current_path = env::current_dir().expect("Error get current directory");
    let header = header_content.replace("#path", current_path.to_str().unwrap());

    println!("{}", header.bold().yellow());
}

pub fn display_install_info() {

    let install_info = get_file_content("install.tmp");

    println!("{}", install_info.bold().cyan());

    get_user_input();
}

pub fn display_install_success() {

    let install_success = get_file_content("install_success.tmp");

    println!("{}", install_success.bold());

    get_user_input();

    exit(0);
}
