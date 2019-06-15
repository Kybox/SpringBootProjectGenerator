use crate::lib::io::global::*;
use std::env;
use std::fs::File;
use std::io::Write;
use crate::PROJECT_DIR;

pub fn create_pom_file(infos: Vec<String>) -> Vec<String> {

    let pom_tmp_file = PROJECT_DIR.get_file("pom.tmp").unwrap();
    let pom_tmp_content = pom_tmp_file.contents_utf8().unwrap();


    // Run the replace operation in memory
    let new_pom_data = pom_tmp_content
        .replace(GROUP_ID, infos.get(0).unwrap())
        .replace(ARTIFACT_ID, infos.get(1).unwrap())
        .replace(VERSION, infos.get(2).unwrap())
        .replace(NAME, infos.get(3).unwrap())
        .replace(DESC, infos.get(4).unwrap())
        .replace(PACKAGING, infos.get(5).unwrap())
        .replace(JAVA, infos.get(6).unwrap());

    // Get the current path
    let mut pom_path = env::current_dir().expect("Error");
    pom_path.push("pom.xml");

    // Recreate the file and dump the processed contents to it
    let mut new_pom = File::create(&pom_path)
        .expect("/!\\ Can't create the pom.xml file");

    new_pom.write(new_pom_data.as_bytes())
        .expect("/!\\ Can't create the pom.xml file");

    println!("[CREATE] {} -> Ok", &pom_path.display());

    let mut app_data = Vec::new();
    app_data.push(infos.get(0).unwrap().to_owned());
    app_data.push(infos.get(1).unwrap().to_owned());

    return app_data;
}
