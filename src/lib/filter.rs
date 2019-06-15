use regex::Regex;

pub fn string_filter(value: String) -> String {

    let reg_exp = Regex::new(r"/[^a-zA-Z ]/g").unwrap();

    return reg_exp.replace_all(value.as_str(), " ").parse().unwrap();
}
