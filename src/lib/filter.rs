use regex::Regex;

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

    if result == false { println!("/!\\ Wrong packaging /!\\ Choose jar or war"); }

    return result;
}

pub fn remove_trailing_newline(mut value: String) -> String {

    value.truncate(value.trim_end_matches(&['\r', '\n'][..]).len());

    return value;
}
