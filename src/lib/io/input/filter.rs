use regex::Regex;
use colored::*;

pub fn string_filter(value: String) -> String {

    let reg_exp = Regex::new(r"/[^a-zA-Z ]/g").unwrap();

    return reg_exp.replace_all(value.as_str(), " ").parse().unwrap();
}

pub fn packaging_filter(value: String) -> bool {

    let result = match value.as_str() {
        "jar" => true,
        "war" => true,
        _ => false,
    };

    if result == false {
        println!("{}", "!! Wrong packaging !!".red().bold());
        println!("{}", "-> Choose jar or war".green().italic());
    }

    return result;
}

pub fn remove_trailing_newline(value: &mut String) -> &String {

    value.truncate(value.trim_end_matches(&['\r', '\n'][..]).len());

    return value;
}
