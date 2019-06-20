use std::process::exit;

use colored::Colorize;

use crate::lib::share::input::get_user_input;
use crate::lib::share::value::*;
use std::io::Error;

pub fn os_version_error() {
    println!("{}", ERROR_OS_VERSION.bold().red());
    println!();
    println!("{}", EXIT_MESSAGE_2.bold().italic());

    get_user_input();

    exit(0);
}

pub fn os_version_not_found() {
    println!("{}", ERROR_OS_NOT_FOUND.bold().red());
    println!();
    println!("{}", EXIT_MESSAGE_2.bold().italic());

    get_user_input();

    exit(0);
}

pub fn error_registry_access() {
    println!("{}", ERROR_INSTALL_1_1.bold().red());
    println!();
    println!("{}", ERROR_INSTALL_1_2.yellow());
    println!("{}", ERROR_INSTALL_1_3.yellow());
    println!();
    println!("{}", EXIT_MESSAGE_2.bold().italic());

    get_user_input();

    exit(0);
}

pub fn error_create_directories(error: Error) {
    println!("{}", ERROR_INSTALL_1_1.bold().red());
    println!();
    println!("{}", ERROR_INSTALL_2_1.yellow());
    println!("{}", error);
    println!();
    println!("{}", EXIT_MESSAGE_2.bold().italic());
}

pub fn error_copy_file(error: Error) {
    println!("{}", ERROR_INSTALL_1_1.bold().red());
    println!();
    println!("{}", ERROR_INSTALL_2_2.yellow());
    println!("{}", ERROR_INSTALL_2_2.yellow());
    println!("{}", error);
    println!();
    println!("{}", EXIT_MESSAGE_2.bold().italic());
}
