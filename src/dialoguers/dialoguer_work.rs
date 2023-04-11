use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use crate::utils::util::{is_valid_package_name, to_valid_package_name};

use crate::ConfiguresSelected;

pub fn work(configures: &mut ConfiguresSelected) -> (String, &ConfiguresSelected) {
    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();

    let mut project_name: String = Input::with_theme(&theme)
        .with_prompt("projectName")
        .default("vue-monorepo-project".to_string())
        .interact_on(&term)
        .unwrap();

    if !is_valid_package_name(&project_name) {
        println!(
            "! Invalid package.json name `{}`, Automatically converted to a valid name.",
            project_name
        );
        project_name = to_valid_package_name(&project_name)
    }
    println!("! Current project name: {}", project_name);

    let config_value: bool = Confirm::with_theme(&theme)
        .with_prompt("Add ESLint for code quality & Add Prettier for code formatting?")
        .interact_on(&term)
        .unwrap();
    configures.set_eslint_config(config_value);

    let config_value: bool = Confirm::with_theme(&theme)
        .with_prompt("Add Vitest for Unit Testing?")
        .interact_on(&term)
        .unwrap();
    configures.set_vitest_config(config_value);

    let config_value: bool = Confirm::with_theme(&theme)
        .with_prompt("Add Common toolbox lib for project?")
        .interact_on(&term)
        .unwrap();
    configures.set_common_toolbox(config_value);

    (project_name, configures)
}
