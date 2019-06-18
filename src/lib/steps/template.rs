use crate::lib::share::value::*;
use dialoguer::{Select};
use dialoguer::theme::ColorfulTheme;

pub fn get_template() -> String {

    return display_template_select();
}

fn display_template_select() -> String {

    let template_list = [
        SIMPLE_PROJECT,
        RESTFULL_WEBSERVICE,
        SOAP_WEBSERVICE
    ];

    println!();
    let selected_template = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(INFO_TEMPLATE_SELECTION)
        .default(0)
        .items(&template_list[..])
        .interact()
        .unwrap();

    return template_list[selected_template].to_string();
}
