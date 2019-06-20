use crate::lib::share::value::*;
use std::env;
use crate::lib::share::filter::remove_line_break;
use crate::lib::share::file::{get_file_content, write_file};

/// This method call all steps for generate the pom.xml file
pub fn create_pom_file(infos: &Vec<String>, template: &String) {

    // Load pom template in memory
    let pom_tmp_content = get_file_content(TMP_POM_FILE);

    // Add basic data
    let pom_basic_content = add_basic_data(infos, &pom_tmp_content);

    // Add dependencies
    let final_pom_content = add_dependencies(pom_basic_content, &template);

    // Write pom file
    write_pom_file(&final_pom_content);
}

fn add_basic_data(infos: &Vec<String>, basic_content: &String) -> String {

    // Run the replace operations in memory
    let new_pom_data = basic_content
        .replace(HASHTAG_GROUP_ID, infos.get(0).unwrap())
        .replace(HASHTAG_ARTIFACT_ID, infos.get(1).unwrap())
        .replace(HASHTAG_VERSION, infos.get(2).unwrap())
        .replace(HASHTAG_NAME, infos.get(3).unwrap())
        .replace(HASHTAG_DESC, infos.get(4).unwrap())
        .replace(HASHTAG_PACKAGING, infos.get(5).unwrap())
        .replace(HASHTAG_JAVA, infos.get(6).unwrap());

    return new_pom_data;
}

fn add_dependencies(pom_content: String, template: &String) -> String {

    let mut content;

    // SIMPLE PROJECT
    // --------------
    if template == SIMPLE_PROJECT {

        return remove_all_hashtags(pom_content);
    }
    else {

        content = add_dependency(&pom_content, HASHTAG_H2DB, H2_TEMP_FILE);
        content = add_dependency(&content, HASHTAG_DATA_JPA, DATA_JPA_TEMP_FILE);

        // REST Webservice
        // ---------------
        if template == RESTFULL_WEBSERVICE {

            content = add_dependency(&content, HASHTAG_DATA_REST, DATA_REST_TEMP_FILE);
            content = remove_hashtag(&content, HASHTAG_WS);
            content = remove_hashtag(&content, HASHTAG_MAVEN_JAXB2_PLUGIN);
        }

        // SOAP Webservice
        // ---------------
        else if template == SOAP_WEBSERVICE {

            content = add_dependency(&content, HASHTAG_WS, WS_TEMP_FILE);
            content = add_dependency(&content, HASHTAG_MAVEN_JAXB2_PLUGIN, MAVEN_JAXB2_PLUGIN_TEMP_FILE);
            content = remove_hashtag(&content, HASHTAG_DATA_REST);
        }

        return content;
    }
}

fn add_dependency(pom_content: &String, tag: &str, file: &str) -> String {

    return pom_content.replace(tag, get_file_content(file).as_str());
}

fn remove_all_hashtags(pom_content: String) -> String {

    return pom_content
        .replace(HASHTAG_H2DB, "")
        .replace(HASHTAG_DATA_JPA, "")
        .replace(HASHTAG_DATA_REST, "")
        .replace(HASHTAG_WS, "")
        .replace(HASHTAG_MAVEN_JAXB2_PLUGIN, "");
}

fn remove_hashtag(pom_content: &String, hashtag: &str) -> String {

    return pom_content.replace(hashtag, "");
}

fn write_pom_file(pom_content: &String) {

    // Filter
    let content = remove_line_break(pom_content);

    // Get the current path
    let mut path = env::current_dir().expect("Error");
    path.push("pom.xml");

    write_file(&path, &content);

    println!("[CREATE] {} -> Ok", &path.display());
}
