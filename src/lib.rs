use regex::Regex;
use std::fs;

pub fn is_valid_package_name(project_name: &str) -> bool {
    Regex::new(r"^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$")
        .unwrap()
        .is_match(project_name)
}

pub fn to_valid_package_name(project_name: &str) -> String {
    let name = project_name
        .trim()
        .to_lowercase()
        .replace(char::is_whitespace, "-")
        .trim_start_matches(|c| c == '.' || c == '_')
        .replace(
            |c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '~'),
            "-",
        );
    name
}

pub fn can_skip_emptying(dir: &str) -> bool {
    if let Ok(metadata) = fs::metadata(dir) {
        if !metadata.is_dir() {
            return false;
        }
    } else {
        return true;
    }

    if let Ok(files) = fs::read_dir(dir) {
        let mut entries = 0;
        for file in files {
            if let Ok(entry) = file {
                if entry.file_name() == ".git" {
                    return entries == 0;
                }
                entries += 1;
            }
        }
        return entries == 0;
    }

    false
}

pub fn empty_dir(dir: &str) {
    if !fs::metadata(dir).is_ok() {
        return;
    }
    println!("Target directory already exists or is not empty, will be reset.");
    fs::remove_dir_all(dir).unwrap();
    fs::create_dir(dir).unwrap();
    // post_order_directory_traverse(dir, |dir| fs::remove_dir(dir), |file| fs::remove_file(file))?;
}
