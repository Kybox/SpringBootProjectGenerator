use crate::lib::share::input::get_user_input;
use colored::Colorize;
use crate::lib::share::filter::{packaging_filter, java_version_filter, match_filter, numeric_start_filter};
use crate::lib::share::value::*;
use crate::lib::share::enums::{MatchType};

///PROJECT BASIC DATA
pub fn get_basic_infos() -> Vec<String> {

    let mut infos = Vec::new();

    infos.push(get_group_id());
    infos.push(get_artifact_id());
    infos.push(get_project_version());
    infos.push(get_project_name());
    infos.push(get_project_description());
    infos.push(get_packaging());
    infos.push(get_java_version());

    return infos;
}

fn get_group_id() -> String {

    let mut group_id = String::new();
    let mut answer_accepted = false;

    while !answer_accepted {

        println!();
        println!("{}", INFO_GROUP_ID.bold());

        group_id = get_user_input().to_lowercase().replace(" ", "");

        let format = match_filter(&group_id, MatchType::GroupId);
        let num_check = numeric_start_filter(&group_id);

        if format && num_check { answer_accepted = true; }

        if !answer_accepted {

            println!("{}", ERROR_GROUP_ID_1_1.red().bold());
            println!("{}", ERROR_GROUP_ID_1_2.green().italic());
        }
    }

    return group_id;
}

fn get_artifact_id() -> String {

    let mut artifact_id = String::new();
    let mut answer_accepted = false;

    while !answer_accepted {

        println!();
        println!("{}", INFO_ARTIFACT_ID.bold());

        artifact_id = get_user_input().to_lowercase().replace(" ", "");

        let format = match_filter(&artifact_id, MatchType::ArtifactId);
        let num_check = numeric_start_filter(&artifact_id);

        if format && num_check { answer_accepted = true; }

        if !answer_accepted {

            println!("{}", ERROR_ARTIFACT_ID_1_1.red().bold());
            println!("{}", ERROR_ARTIFACT_ID_1_2.green().italic());
        }
    }

    return artifact_id;
}

fn get_project_version() -> String {

    let mut version = String::new();

    let mut answer_accepted = false;

    while !answer_accepted {

        println!();
        println!("{}", INFO_PROJECT_VERSION.bold());

        version = get_user_input().to_lowercase().replace(" ", "");

        answer_accepted = !version.is_empty();

        if !answer_accepted {

            println!("{}", ERROR_PROJECT_VERSION_1_1.red().bold());
            println!("{}", ERROR_PROJECT_VERSION_1_2.green().italic());
        }
    }

    return version;
}

fn get_project_name() -> String {

    println!();
    println!("{}", INFO_PROJECT_NAME.bold());

    let mut project_name = get_user_input();

    if project_name.is_empty() {
        project_name = "Project".to_string();
    }

    return project_name;
}

fn get_project_description() -> String {

    println!();
    println!("{}", INFO_PROJECT_DESCRIPTION.bold());

    let mut project_desc = get_user_input();

    if project_desc.is_empty() {

        project_desc = "Project generated with the Spring Boot Project Generator v1.0".to_string();
    }

    return project_desc;
}

fn get_packaging() -> String {

    let mut packaging = String::new();
    let mut answer_accepted = false;

    while !answer_accepted {

        println!();
        println!("{}", INFO_PACKAGING.bold());

        packaging = get_user_input().to_lowercase();
        answer_accepted = packaging_filter(&packaging);

        if !answer_accepted {

            println!("{}", ERROR_PACKAGING_1_1.red().bold());
            println!("{}", ERROR_PACKAGING_1_2.green().italic());
        }
    }

    return packaging;
}

fn get_java_version() -> String {

    let mut version = String::new();
    let mut answer_accepted = false;

    while !answer_accepted {

        println!();
        println!("{}", INFO_JAVA_VERSION.bold());

        version = get_user_input().to_lowercase();
        answer_accepted = java_version_filter(&version);

        if !answer_accepted {

            println!("{}", ERROR_JAVA_VERSION_1_1.red().bold());
            println!("{}", ERROR_JAVA_VERSION_1_2.green().italic());
        }
    }

    return version;
}
