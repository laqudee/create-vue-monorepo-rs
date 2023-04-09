use std::fs;
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

use serde_json::{self, Value};

use super::{deep_merge::merge, sort_dependencied::sort};

pub fn render_template(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    match fs::metadata(src) {
        Ok(stats) if stats.is_dir() => {
            if src.file_name() == Some("node_modules".as_ref()) {
                return Ok(());
            }

            fs::create_dir_all(dest)?;

            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let src_path = entry.path();
                let dest_path = dest.join(entry.file_name());
                render_template(&src_path, &dest_path)?;
            }

            Ok(())
        }
        Ok(_) => {
            // fs::copy()的调用时机，怎么放置，才不会影响package.json的合成
            fs::copy(src, dest)?;

            let file_name = Path::new(src).file_name().unwrap().to_str().unwrap();

            // package.json的排序与合成都有问题
            if let ("package.json", Ok(_)) = (file_name, fs::metadata(&dest)) {
                let existing_contents = fs::read_to_string(&dest).unwrap_or_default();
                let existing: Value = serde_json::from_str(&existing_contents).unwrap();

                let new_contents = fs::read_to_string(&src).unwrap_or_default();
                let new_package: Value = serde_json::from_str(&new_contents).unwrap();

                let mut package_json = merge(&existing, &new_package);
                let pkg = sort(&mut package_json);
                let pkg = serde_json::to_string_pretty(&pkg)?;
                let mut file = fs::File::create(&dest)?;
                file.write_all(pkg.as_bytes())?;
            }

            if file_name.starts_with('_') {
                let parent_dir = dest.parent().unwrap();
                let new_filename = file_name.replacen('_', ".", 1);
                let dest_path = parent_dir.join(new_filename);

                fs::rename(&dest, &dest_path)?;
            }

            if let ("_gitignore", Ok(_)) = (file_name, fs::metadata(&dest)) {
                let existing = fs::read_to_string(dest).unwrap_or_default();
                let new_gitignore = fs::read_to_string(src).unwrap_or_default();
                fs::write(dest, existing + "\n" + &new_gitignore)?;
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}
