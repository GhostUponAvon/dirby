use std::{path::PathBuf, usize};

pub trait Build {
    fn new(size: usize) -> Self;
    fn parse<'a>(file: &'a [String], root_dir: &'a str) -> Result<DirectoryPaths, &'a str>;
    fn get_paths(&self) -> Vec<PathBuf>;

    //may add functionality to return it as a Vector of type String.
}
#[derive(Debug)]
pub struct DirectoryPaths {
    paths: Vec<PathBuf>,
}

impl Build for DirectoryPaths {
    fn new(size: usize) -> DirectoryPaths {
        DirectoryPaths {
            paths: Vec::with_capacity(size),
        }
    }

    fn get_paths(&self) -> Vec<PathBuf> {
        self.paths.clone()
    }

    fn parse<'a>(file: &'a [String], root_dir: &'a str) -> Result<DirectoryPaths, &'a str> {
        let mut paths: DirectoryPaths = DirectoryPaths::new(file.len());
        let mut current_path: PathBuf = PathBuf::new();
        current_path.push(root_dir);
        let mut current_depth: isize = -1;

        for line in file {
            //check depth of the line
            //
            //determine based on depth to move forwards/hold/backwards
            //
            let mut new_depth: isize = 0;
            for chr in line.chars() {
                if chr == '/' {
                    new_depth += 1;
                } else {
                    break;
                }
            }

            match new_depth.cmp(&current_depth) {
                std::cmp::Ordering::Less => {
                    //push the current file path to the paths variable
                    paths.paths.push(current_path.clone());

                    //determine using the current depth how many times to pop the current_path variable
                    for _ in 0..(current_depth - new_depth + 1) {
                        current_path.pop();
                    }
                    current_path.push(&line[new_depth as usize..]);
                }
                std::cmp::Ordering::Equal => {
                    paths.paths.push(current_path.clone());
                    current_path.pop();
                    current_path.push(&line[new_depth as usize..]);
                }
                std::cmp::Ordering::Greater => {
                    current_path.push(&line[new_depth as usize..]);
                }
            }
            current_depth = new_depth;
        }
        paths.paths.push(current_path.clone());

        Ok(paths)
    }
}
