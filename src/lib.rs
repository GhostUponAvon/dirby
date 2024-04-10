use std::env;
pub struct Config {
    execution_path: String,
    input_file: String,
    ouput_dir: String,
}
trait Tree {
    fn has_children(&self) -> Result<usize, &str>;
    fn get_children(&self) -> &Vec<Directory>;
    fn add(&mut self, obj: Directory) -> Result<bool, bool>; //maybe change the return type
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
    fn get_children(&self) -> &Vec<Directory> {
        return &self.children
    }

    fn has_children(&self) -> Result<usize, &str> {
        if self.children.len() == 0 {
            Err("no children found")
        } else {
            Ok(self.children.len())
        }
    }

    fn add(&mut self, obj: Directory) -> Result<bool, bool> {
        self.children.push(obj);
        Ok(true)
    }

    }

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        let input_file: String = args[1].clone();
        let output_dir: String = args[2].clone();
        let execution_path: String = env::current_dir().unwrap().display().to_string();
        
        Ok(Config {input_file: input_file, ouput_dir: output_dir, execution_path: execution_path})
    }
} 