use regex::Regex;
use crate::lib::share::value::*;
use crate::lib::share::enums::MatchType;
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

    let split;

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

pub fn packaging_filter(value: &String) -> bool {

    return match value.as_str() {
        JAR => true,
        WAR => true,
        _ => false
    };

    //return result;
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

pub fn remove_line_break(value: &String) -> String {

    return Regex::new(REGEX_BLANK_LINE)
        .unwrap()
        .replace_all(value, "\n")
        .to_string();
}
