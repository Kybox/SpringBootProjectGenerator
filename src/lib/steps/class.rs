use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use inflector::cases::titlecase::to_title_case;

use crate::lib::share::file::get_file_content;
use crate::lib::share::value::*;

pub fn create_classes(group_id: &String, artifact_id: &String, template: &String) {

    create_main_class(group_id, artifact_id);

    if template == SOAP_WEBSERVICE {
        create_ws_config_class(group_id);
    }
}

fn create_main_class(group_id: &String, artifact_id: &String) {

    let main_class_tmp_content = get_file_content(MAIN_CLASS_TEMP_FILE);

    let mut main_class_content = str::replace(
        to_title_case(artifact_id).as_ref(), " ", "");

    main_class_content.push_str("Application");

    let new_class_content = main_class_tmp_content
        .replace(HASHTAG_GROUP_ID, group_id)
        .replace(HASHTAG_ARTIFACT_ID, &main_class_content);

    let mut user_package = env::current_dir().expect("Error");
    user_package.push(JAVA_PACKAGE);
    user_package.push(get_package_from_group_id(group_id));

    main_class_content.push_str(".java");
    user_package.push(main_class_content.trim());

    // Create the file
    let mut final_main_class = File::create(&user_package)
        .expect("/!\\ Can't create the main java class");

    // Dump the processed contents to it
    final_main_class.write(new_class_content.as_bytes())
        .expect("/!\\ Can't write content the main java class");

    println!("[CREATE] {} -> Ok", &user_package.display());
}

fn create_ws_config_class(group_id: &String) {

    let tmp_config_class_content = get_file_content(WS_CONFIG_CLASS_TEMP_FILE);
    let config_class_content = tmp_config_class_content
        .replace(HASHTAG_GROUP_ID, group_id);

    let mut config_file = env::current_dir().expect("Error");
    config_file.push(JAVA_PACKAGE);
    config_file.push(get_package_from_group_id(group_id));
    config_file.push(SOAP_WS_CONFIG_CLASS);

    // Create the file
    let mut final_config_file = File::create(&config_file)
        .expect("/!\\ Can't create the java class");

    // Dump the processed contents to it
    final_config_file.write(config_class_content.as_bytes())
        .expect("/!\\ Can't create the java class");

    println!("[CREATE] {} -> Ok", config_file.display());
}

fn get_package_from_group_id(group_id: &String) -> PathBuf {

    let mut full_package = PathBuf::new();

    for p in group_id.split(".") {
        full_package.push(p);
    }

    return full_package;
}
