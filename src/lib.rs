use std::{env, fs, path::Path};

pub struct Config {
    pub execution_path: String,
    pub input_file: String,
    pub ouput_dir: String,
}
trait Tree {
    fn leaves(&self) -> &Vec<String>;
    fn children(&self) -> &Vec<Directory>;
}
///Object representing a filesystem folder
pub struct Directory {
    name: String,
    children: Vec<Directory>,
    has_children: bool,
}

///Object representing the directory the program was called inside.
///All child directories will be made inside of this directory
pub struct RootDirectory {
    name: String,
    children: Vec<Directory>,
    has_children: bool,
}

impl Tree for RootDirectory {
    fn children(&self) -> &Vec<Directory> {
        return &self.children
    }

    fn leaves(&self) -> &Vec<String> {
        let leaves = Vec::new();
        for child in self.children {
            
        }
    }
}




impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        let input_file: String = args[1].clone();
        let output_dir: String = args[2].clone();
        
        //FILL
    }
} 