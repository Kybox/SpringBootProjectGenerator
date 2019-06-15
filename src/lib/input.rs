use crate::lib::filter::{string_filter, packaging_filter, remove_trailing_newline};
use std::io;

pub fn get_infos() -> Vec<String> {

    let mut infos = Vec::new();

    println!("Enter the groupId :");
    infos.push(read_line());

    println!("Enter the artifactId :");
    infos.push(string_filter(read_line()));

    println!("Enter the project version :");
    infos.push(read_line());

    println!("Enter the name of the project :");
    infos.push(read_line());

    println!("Enter the project description :");
    infos.push(read_line());

    let packaging_info = "Enter the packaging (jar or war) :";
    let mut done = false;
    while !done {
        println!("{}", packaging_info);
        done = packaging_filter(remove_trailing_newline(read_line()));
    }

    //if read_packaging() { infos.push(read_line()); }
    //else { println!("{}", packaging_info); }

    println!("Enter the Java version :");
    infos.push(read_line());


    // Remove line breaks
    for info in infos.iter_mut() {
        info.truncate(info.trim_end_matches(&['\r', '\n'][..]).len());
    }

    return infos;
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


