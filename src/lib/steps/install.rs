use dirs::data_local_dir;
use crate::lib::share::value::*;
use std::fs::{create_dir, copy};
use std::path::PathBuf;
use winreg::{RegKey, HKEY};
use winreg::enums::HKEY_LOCAL_MACHINE;
use std::process::Command;
use crate::lib::steps::error::{error_registry_access, error_create_directories, error_copy_file};
use crate::lib::banner::display_install_success;

pub fn check_install() -> bool {

    return check_user_path();
}

fn check_user_path() -> bool {

    let path_content = get_path_content();
    let install_path = get_installation_path();

    if path_content.contains(install_path.as_str()) { return true; }
    else { return false; }
}

fn get_path_content() -> String {

    let user_path = get_regkey(HKEY_LOCAL_MACHINE, REGSAM_KEY_READ);
    let mut path_content = String::new();

    match user_path.get_value(PATH_ENV_KEY) {
        Ok(data) => path_content = data,
        Err(e) => println!("Error read PATH content : {}", e)
    };

    return path_content;
}

fn get_installation_path() -> String {

    let mut install_dir = PathBuf::new();
    install_dir.push(data_local_dir().expect("Can't find AppData directory"));
    install_dir.push(INSTALL_DIR);

    let full_path = install_dir.to_str().unwrap().to_owned();

    return full_path;
}

pub fn install() {

    check_admin_mode();
    copy_application();
    update_path_env();
    display_install_success();
}

fn copy_application() {

    let mut install_dir = PathBuf::new();
    install_dir.push(&get_installation_path());

    match create_dir(&install_dir) {
        Ok(_result) => (),
        Err(e) => error_create_directories(e)
    }

    install_dir.push(APP_FILE_NAME);

    match copy(APP_FILE_NAME, &install_dir) {
        Ok(_) => (),
        Err(e) => error_copy_file(e)
    }
}

fn update_path_env() {

    let mut path_content = String::new();
    path_content.push_str(get_installation_path().as_str());
    path_content.push_str(";");
    path_content.push_str(get_path_content().as_str());

    let path_regkey = get_regkey(HKEY_LOCAL_MACHINE, REGSAM_KEY_SET_VALUE);

    match path_regkey.set_value(PATH_ENV_KEY, &path_content) {
        Ok(_) => (),
        Err(e) => println!("Error during Path var update : {}", e)
    }

    let kill = "taskkill /F /IM explorer.exe & start explorer";

    Command::new("cmd")
        .args(&["/C", &kill])
        .output()
        .expect("Failed to refresh the environment variables");
}

fn check_admin_mode() {

    let _registry_key = get_regkey(HKEY_LOCAL_MACHINE, REGSAM_KEY_SET_VALUE);
}

fn get_regkey(hkey: HKEY, regsam: u32) -> RegKey {

    let regkey = RegKey::predef(hkey);

       match regkey.open_subkey_with_flags(REG_ENVIRONMENT_HKEY, regsam) {
           Ok(data) => return data,
           Err(_e) => {
               error_registry_access();
           }
       };

    return regkey;
}


