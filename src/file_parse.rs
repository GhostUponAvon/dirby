use dirby::{RootDirectory, Directory};
use std::path::PathBuf;

trait Creation {
    fn parse(&self, file: &String) -> Result<DirectoryPaths, &str>;
    fn length(&self) -> usize;
    fn get_paths(&self) -> Vec<PathBuf>;
    //may add functionality to return it as a Vector of type String.
}
struct DirectoryPaths {
    length: usize,
    paths: Vec<PathBuf>
}

impl Creation for DirectoryPaths {
    fn length(&self) -> usize {
        return self.length.clone();
    }

    fn get_paths(&self) -> Vec<PathBuf> {
        return self.paths.clone()
    }

    //add fn parse 
}

