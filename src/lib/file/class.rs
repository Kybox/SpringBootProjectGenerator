use crate::PROJECT_DIR;
use crate::lib::file::global::*;
use inflector::cases::titlecase::to_title_case;
use std::env;
use std::fs::File;
use std::io::Write;

pub fn create_main_class(package: &String, artifact_id: &String) {

    let class_tmp_file = PROJECT_DIR.get_file("class.tmp").unwrap();
    let class_tmp_content = class_tmp_file.contents_utf8().unwrap();

    let mut class_name = str::replace(to_title_case(artifact_id)
                                          .as_ref(), " ", "");

    class_name.push_str("Application");

    let new_class_content = class_tmp_content
        .replace(GROUP_ID, package)
        .replace(ARTIFACT_ID, &class_name);

    let mut user_package = env::current_dir().expect("Error");
    user_package.push("src\\main\\java");

    for p in package.split("."){
        user_package.push(p);
    }

    class_name.push_str(".java");
    user_package.push(class_name.trim());


    // Recreate the file and dump the processed contents to it
    let mut new_class_file = File::create(&user_package)
        .expect("/!\\ Can't create the java class");

    new_class_file.write(new_class_content.as_bytes())
        .expect("/!\\ Can't create the java class");

    println!("[CREATE] {} -> Ok", &user_package.display());
}
