use git2::Repository;
use serde_json::json;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use create_vue_monorepo_rs::dialoguers::dialoguer_work;
use create_vue_monorepo_rs::dialoguers::termcolor_work;
use create_vue_monorepo_rs::render::main_render::render;
use create_vue_monorepo_rs::utils::util::empty_dir;
use create_vue_monorepo_rs::ConfiguresSelected;

fn main() -> std::io::Result<()> {
    let mut configures_selected = ConfiguresSelected::default();

    let (project_name, configures_selected) = dialoguer_work::work(&mut configures_selected);

    let root = env::current_dir().unwrap().join(project_name.clone());

    // Init git
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

    // Decorating command line output
    termcolor_work::work(&project_name, &configures_selected)?;

    Ok(())
}
