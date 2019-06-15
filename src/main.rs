extern crate dirs;
extern crate inflector;

#[macro_use]
extern crate include_dir;
use include_dir::Dir;
pub const PROJECT_DIR: Dir = include_dir!("resources");

mod lib;

fn main() {

    lib::banner::display_banner();

    let infos = lib::input::get_infos();
    let app_data = lib::file::pom::create_pom_file(infos);

    lib::file::folder::create_directories(app_data.get(0).unwrap());
    lib::file::class::create_main_class(app_data.get(0).unwrap(), app_data.get(1).unwrap());
}
