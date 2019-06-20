use std::env;
use crate::lib::share::value::*;
use crate::lib::share::file::{get_file_content, write_file};

pub fn create_properties_file(template: &String) {

    let mut properties_content = get_file_content(PROPERTIES_TEMP_FILE);

    if template != SIMPLE_PROJECT {

        let h2db_properties = get_file_content(H2DB_PROPERTIES_TEMP_FILE);
        properties_content = properties_content.replace(HASHTAG_H2DB, &h2db_properties);
    }
    else {
        properties_content = properties_content.replace(HASHTAG_H2DB, "");
    }

    let mut path = env::current_dir().expect("Error");
    path.push(RESOURCES_FOLDER);
    path.push(PROPERTIES_FILE);

    write_file(&path, &properties_content);

    println!("[CREATE] {} -> Ok", path.display());
}
