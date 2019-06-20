use crate::lib::share::value::{EXIT_MESSAGE_1, EXIT_MESSAGE_2};
use crate::lib::share::input::get_user_input;
use colored::Colorize;

pub fn exit_generator() {

    println!();
    println!("{}", EXIT_MESSAGE_1.bold().yellow());
    println!("{}", EXIT_MESSAGE_2.bold().green());

    get_user_input();
}
