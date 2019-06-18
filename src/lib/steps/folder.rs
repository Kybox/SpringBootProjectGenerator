use crate::lib::share::value::{JAVA_PACKAGE, SOAP_WEBSERVICE};
use std::env::current_dir;
use std::fs::create_dir_all;

pub fn create_directories(group_id: &String, template: &String) {

    let mut user_package = current_dir().expect("Error");
    user_package.push(JAVA_PACKAGE);

    for p in group_id.split("."){
        user_package.push(p);
    }

    create_dir_all(&user_package).expect("Error while create dir");
    println!("[CREATE] {} -> Ok", &user_package.display());

    let mut resource_path = current_dir().expect("Error");
    resource_path.push("src\\main\\resources");
    create_dir_all(&resource_path).expect("Error while create dir");
    println!("[CREATE] {} -> Ok", &resource_path.display());

    if template == SOAP_WEBSERVICE {

        let mut xsd_path = current_dir().expect("Error");
        xsd_path.push("src\\main\\resources\\xsd");
        create_dir_all(&xsd_path).expect("Error while create dir");
        println!("[CREATE] {} -> Ok", &xsd_path.display());

        user_package.push("config");
        create_dir_all(&user_package).expect("Error while create dir");
        println!("[CREATE CONFIG] {} -> Ok", &user_package.display());
    }

    let mut test_path = current_dir().expect("Error");
    test_path.push("src\\test\\java");
    create_dir_all(&test_path).expect("Error while create dir");
    println!("[CREATE] {} -> Ok", &test_path.display());
}
