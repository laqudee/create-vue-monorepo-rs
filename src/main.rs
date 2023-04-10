mod utils;

use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use git2::Repository;
use serde_json::json;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use create_vue_monorepo_rs::{empty_dir, is_valid_package_name, to_valid_package_name};
use utils::render::render_template;

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

fn main() -> std::io::Result<()> {
    let mut configures_selected = ConfiguresSelected::default();

    let (project_name, configures_selected) = dialoguer_work(&mut configures_selected);

    let root = env::current_dir().unwrap().join(project_name.clone());

    // 初始化git
    let _repo = match Repository::init(&root) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    if fs::metadata(root.clone()).is_ok() {
        empty_dir(root.as_path().to_str().unwrap());
    } else if fs::metadata(root.clone()).is_err() {
        fs::create_dir(root.clone()).unwrap();
    }

    println!("! target dir: {:?}", root.display());
    if project_name.len() != 0 {
        println!("! project name: {}", project_name);
    }

    let pkg = json!({
        "name": project_name,
        "version": "0.0.1"
    });

    let mut file = fs::File::create(format!("{}/package.json", root.as_path().to_str().unwrap()))
        .expect("Unable to create file");
    file.write_all(pkg.to_string().as_bytes())
        .expect("Unable to write data to file");

    let template_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("template");

    render("base", &template_root, &root)?;

    if configures_selected.vitest_config {
        render("config/vitest", &template_root, &root)?;
    }

    if configures_selected.common_toolbox {
        render("code", &template_root, &root)?;
    }

    if configures_selected.eslint_config {
        render("config/eslint", &template_root, &root)?;
    }

    Ok(())
}

pub fn render(template_name: &str, template_root: &PathBuf, root: &PathBuf) -> std::io::Result<()> {
    let template_dir = template_root.join(template_name);
    render_template(&template_dir, root)?;

    Ok(())
}

pub fn dialoguer_work(configures: &mut ConfiguresSelected) -> (String, &ConfiguresSelected) {
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
