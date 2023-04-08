use regex::Regex;
use std::fs;

pub fn is_valid_package_name(project_name: &str) -> bool {
    let re = Regex::new(r"/^(?:@[a-z0-9-*~][a-z0-9-*._~]*\/)?[a-z0-9-~][a-z0-9-._~]*$/").unwrap();
    re.is_match(project_name)
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

pub fn empty_dir(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !fs::metadata(dir)?.is_dir() {
        return Ok(());
    }

    post_order_directory_traverse(dir, |dir| fs::remove_dir(dir), |file| fs::remove_file(file))?;
    Ok(())
}

pub fn post_order_directory_traverse<F, G>(path: &str, dir_cb: F, file_cb: G) -> std::io::Result<()>
where
    F: Fn(&str) -> std::io::Result<()>,
    G: Fn(&str) -> std::io::Result<()>,
{
    if fs::metadata(path)?.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                post_order_directory_traverse(&path.to_string_lossy(), &dir_cb, &file_cb)?;
            } else {
                file_cb(&path.to_string_lossy())?;
            }
        }
        dir_cb(path)?;
        Ok(())
    } else {
        file_cb(path)?;
        Ok(())
    }
}
