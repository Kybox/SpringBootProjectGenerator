use regex::Regex;
use colored::*;
use crate::lib::share::value::*;
use crate::lib::share::enums::{CharType, CharCase, MatchType};
use crate::lib::share::regex::*;

pub fn match_filter(value: &String, filter: MatchType) -> bool {

    match filter {
        MatchType::GroupId => {
            return Regex::new(REGEX_GROUP_ID).unwrap().is_match(value.as_str());
        }
        MatchType::ArtifactId => {
            return Regex::new(REGEX_ARTIFACT_ID).unwrap().is_match(value.as_str());
        }
    };
}

pub fn numeric_start_filter(value: &String) -> bool {

    let mut result = true;

    let mut split = "";

    if value.contains(".") { split = "."; }
    else if value.contains("-") { split = "-"; }
    else { return false; }

    for data in value.split(split){

        let word = String::from(data);

        if word.is_empty() || word.chars().next().unwrap().is_numeric() {
            result = false;
            break;
        }
    }

    return result;
}

pub fn restrict_char_filter(value: String, format:CharType, case: CharCase) -> String {

    let mut typed_value = String::new();

    match format {

        CharType::NUMERIC => {
            typed_value = Regex::new("\\P{N}")
                .unwrap()
                .replace_all(&value, "")
                .to_string();
        },
        CharType::LETTER => {
            typed_value = Regex::new("\\P{L}")
                .unwrap()
                .replace_all(&value, "")
                .to_string();
        },
        CharType::ALL => typed_value = value.replace(" ", "")
    };

    println!("Result -> {}", typed_value);

    match case {

        CharCase::UPPERCASE => return typed_value.to_uppercase(),
        CharCase::LOWERCASE => return typed_value.to_lowercase()
    };
}

pub fn packaging_filter(value: &String) -> bool {

    let result = match value.as_str() {
        JAR => true,
        WAR => true,
        _ => false
    };

    return result;
}

pub fn java_version_filter(value: &String) -> bool {

    let result = match value.as_str() {

        JAVA_8 => true,
        JAVA_11 => true,
        JAVA_12 => true,
        _ => false
    };

    return result;
}

pub fn remove_line_break(value: String) -> String {

    return Regex::new(REGEX_BLANK_LINE)
        .unwrap()
        .replace_all(&value.as_str(), "\n")
        .to_string();
}

pub fn yes_no_filter(value: String) -> bool {

    let result = match value.as_str() {
        YES => true,
        NO => true,
        _ => false,
    };

    if result == false {
        println!("{}", "!! Wrong answer !!".red().bold());
        println!("{}", "-> Choose y for yes or n for no".green().italic());
    }

    return result;
}
