use dirby::{Directory, Tree};
use std::{path::PathBuf, usize};


pub trait Build {
    fn new(size: usize) -> DirectoryPaths;
    fn parse(file: &Vec<String>, root_dir: &str) -> Result<DirectoryPaths, &str>;
    fn length(&self) -> usize;
    fn get_paths(&self) -> Vec<PathBuf>;

    //may add functionality to return it as a Vector of type String.
}
pub struct DirectoryPaths {
    length: usize,
    paths: Vec<PathBuf>
}

impl Build for DirectoryPaths {
    
    fn new(size: usize) -> DirectoryPaths {
        DirectoryPaths {length: 0, paths: Vec::with_capacity(size)}
    }
    
    fn length(&self) -> usize {
        return self.length.clone();
    }

    fn get_paths(&self) -> Vec<PathBuf> {
        return self.paths.clone()
    }

    fn parse(file: &Vec<String>, root_dir: &str) -> Result<DirectoryPaths, &str> {
        
        let mut paths: DirectoryPaths = DirectoryPaths::new(file.len());
        let mut root_tree: Directory = Directory {name: root_dir.to_owned(), children: Vec::new(), has_children: false}

        let mut current_node:  Directory = root_tree;
        let mut previous_node: Directory;
        let mut current_depth: String = "".to_owned();
        for line in file { 

            if line[current_depth.len()..].contains("/") {
                current_depth += "/";
                let mut new_node: Directory = Directory {name: line[current_depth.len()..].to_string(), children: Vec::new(), has_children: false};
                current_node.has_children = true;
                current_node.add(new_node);
                previous_node = current_node;
                current_node = new_node;
            } 

            //continue here and add handler for when there isn't a new sub node

            
            //delegate job to a function responsible for building the directory tree.
        }
        
    
    todo!()
    }
}

