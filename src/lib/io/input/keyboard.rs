use crate::lib::io::input::filter::{string_filter, packaging_filter, remove_trailing_newline};
use std::io;
use colored::Colorize;

pub fn get_infos() -> Vec<String> {

    let mut infos = Vec::new();

    println!();
    println!("{}", "Enter the groupId :".bold());
    infos.push(read_line());

    println!();
    println!("{}", "Enter the artifactId :".bold());
    infos.push(string_filter(read_line()));

    println!();
    println!("{}", "Enter the project version :".bold());
    infos.push(read_line());

    println!();
    println!("{}", "Enter the name of the project :".bold());
    infos.push(read_line());

    println!();
    println!("{}", "Enter the project description :".bold());
    infos.push(read_line());

    infos.push(get_packaging());

    println!();
    println!("{}", "Enter the Java version :".cyan().bold());
    infos.push(read_line());


    // Remove line breaks
    for info in infos.iter_mut() {
        info.truncate(info.trim_end_matches(&['\r', '\n'][..]).len());
    }

    return infos;
}

fn get_packaging() -> String {

    let packaging_info = "Enter the packaging (jar or war) :";
    let mut packaging = String::new();
    let mut done = false;
    while !done {
        println!();
        println!("{}", packaging_info.bold());

        packaging = read_line();
        done = packaging_filter(remove_trailing_newline(&mut packaging).parse().unwrap());
    }

    return packaging;
}

fn read_line() -> String {

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {

        Ok(_) => { return input },
        Err(e) => {
            print!("Oops something went wrong : {}", e);
            return String::from("Error");
        }
    }
}


