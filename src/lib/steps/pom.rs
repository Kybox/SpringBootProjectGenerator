use crate::lib::share::value::*;
use std::env;
use std::fs::File;
use std::io::Write;
use crate::lib::share::filter::remove_line_break;
use crate::lib::share::file::get_file_content;

/// This method call all steps for generate the pom.xml file
pub fn create_pom_file(infos: &Vec<String>, template: &String) {

    // Load pom template in memory
    let pom_tmp_content = get_file_content(TMP_POM_FILE);

    // Add basic data
    let pom_basic_content = add_basic_data(infos, &pom_tmp_content);

    // Add dependencies
    let final_pom_content = add_dependencies(pom_basic_content, &template);

    // Write pom file
    write_pom_file(final_pom_content);
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

    let mut final_pom = String::new();

    // SIMPLE PROJECT
    // --------------
    if template == SIMPLE_PROJECT {

        final_pom = remove_all_hashtags(pom_content);
        return final_pom;
    }
    else {

        final_pom = add_dependency(
            pom_content,
            HASHTAG_H2DB, H2_TEMP_FILE);

        final_pom = add_dependency(
            final_pom,
            HASHTAG_DATA_JPA, DATA_JPA_TEMP_FILE);

        // REST Webservice
        // ---------------
        if template == RESTFULL_WEBSERVICE {

            final_pom = add_dependency(
                final_pom,
                HASHTAG_DATA_REST, DATA_REST_TEMP_FILE);

            final_pom = remove_hashtag(
                final_pom,
                HASHTAG_WS);

            final_pom = remove_hashtag(
                final_pom,
                HASHTAG_MAVEN_JAXB2_PLUGIN);
        }

        // SOAP Webservice
        // ---------------
        else if template == SOAP_WEBSERVICE {

            final_pom = add_dependency(
                final_pom,
                HASHTAG_WS, WS_TEMP_FILE);

            final_pom = add_dependency(
                final_pom,
                HASHTAG_MAVEN_JAXB2_PLUGIN, MAVEN_JAXB2_PLUGIN_TEMP_FILE);

            final_pom = remove_hashtag(
                final_pom,
                HASHTAG_DATA_REST);
        }

        return final_pom;
    }
}

fn add_dependency(pom_content: String, tag: &str, file: &str) -> String {

    let pom_updated = pom_content.replace(tag, get_file_content(file).as_str());

    return pom_updated;
}

fn remove_all_hashtags(pom_content: String) -> String {

    let updated_pom = pom_content
        .replace(HASHTAG_H2DB, "")
        .replace(HASHTAG_DATA_JPA, "")
        .replace(HASHTAG_DATA_REST, "")
        .replace(HASHTAG_WS, "");

    return updated_pom;
}

fn remove_hashtag(pom_content: String, hashtag: &str) -> String {

    let updated_pom = pom_content.replace(hashtag, "");

    return updated_pom;
}

fn write_pom_file(final_pom_content: String) {

    // Filter
    let content_to_write = remove_line_break(final_pom_content);

    // Get the current path
    let mut pom_path = env::current_dir().expect("Error");
    pom_path.push("pom.xml");

    // Create the file
    let mut new_pom = File::create(&pom_path)
        .expect("/!\\ Can't create the pom.xml file");

    // Dump the processed contents to it
    new_pom.write(content_to_write.as_bytes())
        .expect("/!\\ Can't create the pom.xml file");

    println!("[CREATE] {} -> Ok", &pom_path.display());
}
