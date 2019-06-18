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

    lib::banner::display_banner();
    lib::banner::display_header();

    test();

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

    /*
    let dependency_map = lib::io::input::dependency::get_dependencies();

    let app_data = lib::io::output::pom::create_pom_file(&infos, &dependency_map);

    let group_id = app_data.get(0).unwrap();
    let artifact_id = app_data.get(1).unwrap();

    lib::io::output::folder::create_directories(group_id);
    lib::io::output::class::create_main_class(group_id, artifact_id);
    */
}
fn test() {



}
