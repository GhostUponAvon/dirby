// will handle the actual directory manipulation
use std::{fs, path::PathBuf};

fn build_fs_tree(fs_tree: Vec<PathBuf>) -> Result<bool, &str> {
    for path in fs_tree {
        fs::create_dir_all(path.as_path());
    }
    todo!()
}
