use crate::lib::share::value::OS_WINDOWS_10;
use crate::lib::steps::error::{os_version_error, os_version_not_found};

pub fn check_os_version() {

    match os_info::get().version().edition() {
        Some(version) => {
            if !version.starts_with(OS_WINDOWS_10) {
                os_version_error();
            }
        },
        None => os_version_not_found()
    }

}
