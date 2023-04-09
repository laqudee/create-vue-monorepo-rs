use std::fs;
use std::io::Result;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::{Value,self};

use super::{sort_dependencied::sort, deep_merge::merge};

pub fn render_template(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    let stats = fs::metadata(src).unwrap();
    if stats.is_dir() {
        if src.file_name() == Some("node_modules".as_ref()) {
            return Ok(());
        }

        // 如果是个目录，则递归渲染其内部的子文件
        fs::create_dir_all(dest).unwrap();
        for entry in fs::read_dir(src).unwrap() {
            let entry = entry.unwrap();
            let src_path = entry.path();
            let dest_path = dest.join(entry.file_name());
            render_template(&src_path, &dest_path)?;
        }
        return Ok(());
    }

    fs::copy(src, dest)?;

    let file_name = Path::new(src).file_name().unwrap().to_str().unwrap();

    // TODO: 处理 package.json 文件
    if file_name == "package.json" && fs::metadata(&dest).is_ok() {
      let existing_contents = fs::read_to_string(&dest).unwrap_or_default();
      let existing: Value = serde_json::from_str(&existing_contents).unwrap();

      let new_contents = fs::read_to_string(&src).unwrap_or_default();
      let new_package: Value = serde_json::from_str(&new_contents).unwrap();

      let mut package_json = merge(&existing, &new_package);
      let pkg= sort(&mut package_json);
      let pkg = serde_json::to_string_pretty(&pkg)?;
      let mut file = fs::File::create(&dest)?;
      file.write_all(pkg.as_bytes())?;
      return Ok(());
    }

    if file_name.starts_with('_') {
        let parent_dir = dest.parent().unwrap();
        let new_filename = file_name.replacen('_', ".", 1);
        let dest_path = parent_dir.join(new_filename);

        fs::rename(&dest, &dest_path)?;
    }

    if file_name == "_gitignore" && fs::metadata(dest).is_ok() {
        let existing = fs::read_to_string(dest).unwrap_or_default();
        let new_gitignore = fs::read_to_string(src).unwrap_or_default();
        fs::write(dest, existing + "\n" + &new_gitignore).unwrap();
        return Ok(());
    }

    Ok(())
}
