use std::fs;
use std::path::PathBuf;

pub fn handle_target_dir(root: &PathBuf) {
    if fs::metadata(&root).is_ok() {
        empty_dir(&root).unwrap();
    } else {
        fs::create_dir_all(&root).unwrap();
    }
}

fn empty_dir(root: &PathBuf) -> std::io::Result<()> {
    let entries = fs::read_dir(&root)?;

    for entry in entries {
        let entry = entry?;

        // If the entry is a directory, recursively remove its contents
        if entry.file_type()?.is_dir() {
            empty_dir(&entry.path())?;
        }

        fs::remove_file(entry.path())?;
    }

    Ok(())
}
