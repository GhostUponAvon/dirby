// will handle the actual directory manipulation
use std::{fs, path::PathBuf};

pub fn build_fs_tree(fs_tree: Vec<PathBuf>) -> std::io::Result<()> {
    for path in fs_tree {
        //add Error handling for directories that couldn't be created. 
        fs::create_dir_all(path.as_path())?;
    }
    Ok(())
}
