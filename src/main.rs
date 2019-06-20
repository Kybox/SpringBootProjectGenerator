extern crate dirs;
extern crate inflector;

#[macro_use]
extern crate include_dir;

use include_dir::Dir;
use crate::lib::steps::project::get_basic_infos;
use crate::lib::steps::template::get_template;
use crate::lib::steps::pom::*;
use crate::lib::steps::folder::create_directories;
use crate::lib::steps::class::{create_classes};
use crate::lib::steps::exit::exit_generator;
use crate::lib::steps::properties::create_properties_file;
use crate::lib::steps::install::{check_install, install};
use crate::lib::banner::display_install_info;
use crate::lib::steps::version::check_os_version;

pub const PROJECT_DIR: Dir = include_dir!("resources");

mod lib;

fn main() {

    #[cfg(windows)] unsafe {
        let handle = winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_OUTPUT_HANDLE);
        let mut original_mode: winapi::shared::minwindef::DWORD = 0;
        winapi::um::consoleapi::GetConsoleMode(handle, &mut original_mode);
        winapi::um::consoleapi::SetConsoleMode(handle, winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING | original_mode);
    }

    // Clear screen
    print!("{}[2J", 27 as char);

    test();
    // Display banner
    lib::banner::display_banner();

    // Check OS Version
    check_os_version();

    // Check install
    if !check_install() {

        display_install_info();
        install();
    }


    lib::banner::display_header();

    // Get basic project infos
    let infos = get_basic_infos();
    let group_id = infos.get(0).unwrap();
    let artifact_id = infos.get(1).unwrap();

    // Get template list
    let template = get_template();

    // Create pom file
    create_pom_file(&infos, &template);

    // Create directories
    create_directories(group_id, &template);

    // Create classes
    create_classes(group_id, artifact_id, &template);

    // Create the properties file
    create_properties_file(&template);

    exit_generator();
}

fn test() {


}
