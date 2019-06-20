use std::env;
use std::path::PathBuf;

use inflector::cases::titlecase::to_title_case;

use crate::lib::share::file::{get_file_content, write_file};
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

    let class_content = main_class_tmp_content
        .replace(HASHTAG_GROUP_ID, group_id)
        .replace(HASHTAG_ARTIFACT_ID, &main_class_content);

    let mut path = env::current_dir().expect("Error");
    path.push(JAVA_PACKAGE);
    path.push(get_package_from_group_id(group_id));

    main_class_content.push_str(".java");
    path.push(main_class_content.trim());

    write_file(&path, &class_content);

    println!("[CREATE] {} -> Ok", &path.display());
}

fn create_ws_config_class(group_id: &String) {

    let tmp_config_class_content = get_file_content(WS_CONFIG_CLASS_TEMP_FILE);
    let config_class_content = tmp_config_class_content
        .replace(HASHTAG_GROUP_ID, group_id);

    let mut path = env::current_dir().expect("Error");
    path.push(JAVA_PACKAGE);
    path.push(get_package_from_group_id(group_id));
    path.push(SOAP_WS_CONFIG_CLASS);

    write_file(&path, &config_class_content);

    println!("[CREATE] {} -> Ok", path.display());
}

fn get_package_from_group_id(group_id: &String) -> PathBuf {

    let mut full_package = PathBuf::new();

    for p in group_id.split(".") {
        full_package.push(p);
    }

    return full_package;
}
