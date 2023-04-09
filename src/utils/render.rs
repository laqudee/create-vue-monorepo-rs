use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

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

    // package.json 文件尚未处理
    // TODO: 处理 package.json 文件
    
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
