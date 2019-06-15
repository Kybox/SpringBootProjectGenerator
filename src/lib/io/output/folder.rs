use std::{env, fs};

pub fn create_directories(package: &String) {

    let mut user_package = env::current_dir().expect("Error");
    user_package.push("src\\main\\java");

    for p in package.split("."){
        user_package.push(p);
    }

    fs::create_dir_all(&user_package).expect("Error while create dir");
    println!("[CREATE] {} -> Ok", &user_package.display());

    let mut resource_path = env::current_dir().expect("Error");
    resource_path.push("src\\main\\resources");
    fs::create_dir_all(&resource_path).expect("Error while create dir");
    println!("[CREATE] {} -> Ok", &resource_path.display());

    let mut test_path = env::current_dir().expect("Error");
    test_path.push("src\\test\\java");
    fs::create_dir_all(&test_path).expect("Error while create dir");
    println!("[CREATE] {} -> Ok", &test_path.display());
}
