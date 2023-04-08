use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::env;
use std::fs;

// use create_vue_monorepo_rs::{
//     can_skip_emptying, empty_dir, is_valid_package_name, to_valid_package_name,
// };

#[derive(Debug)]
pub struct ConfiguresSelected {
    pub eslint_config: bool,
    pub prettier_config: bool,
    pub vitest_config: bool,
    pub common_toolbox: bool,
}

impl ConfiguresSelected {
    pub fn new() -> Self {
        Self {
            eslint_config: false,
            prettier_config: false,
            vitest_config: false,
            common_toolbox: false,
        }
    }

    pub fn set_eslint_config(&mut self, value: bool) {
        self.eslint_config = value
    }

    pub fn set_prettier_config(&mut self, value: bool) {
        self.prettier_config = value
    }

    pub fn set_vitest_config(&mut self, value: bool) {
        self.vitest_config = value
    }

    pub fn set_common_toolbox(&mut self, value: bool) {
        self.common_toolbox = value
    }
}

impl Default for ConfiguresSelected {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut configures_selected = ConfiguresSelected::default();

    let (project_name, configures_selected) = dialoguer_work(&mut configures_selected);

    if project_name.len() != 0 {
        println!("project name: {}", project_name);
    }
    println!("The selected configures: {:?}", configures_selected);
}

pub fn dialoguer_work(configures: &mut ConfiguresSelected) -> (String, &ConfiguresSelected) {
    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();

    let project_name: String = Input::with_theme(&theme)
        .with_prompt("projectName")
        .default("vue-monorepo-project".to_string())
        .interact_on(&term)
        .unwrap();

    let root = env::current_dir().unwrap().join(project_name.clone());
    println!("root: {:?}", root.display());
    if fs::metadata(root.clone()).is_ok() {
        fs::remove_dir_all(root.clone()).unwrap();
    } else if fs::metadata(root.clone()).is_err() {
        fs::create_dir(root.clone()).unwrap();
    }

    let config_value: bool = Confirm::with_theme(&theme)
        .with_prompt("Add ESLint for code quality?")
        .interact_on(&term)
        .unwrap();
    configures.set_eslint_config(config_value);

    let config_value: bool = Confirm::with_theme(&theme)
        .with_prompt("Add Prettier for code formatting?")
        .interact_on(&term)
        .unwrap();
    configures.set_prettier_config(config_value);

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
