use crate::lib::filter::string_filter;
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

    println!("Enter the packaging (WAR or JAR) :");
    infos.push(read_line());

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
