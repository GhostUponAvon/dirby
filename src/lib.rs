use std::env;
pub struct Config {
    pub execution_path: String,
    pub input_file: String,
    pub output_dir: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let input_file: String = args[1].clone();
        let output_dir: String = args[2].clone();
        let execution_path: String = env::current_dir().unwrap().display().to_string();

        Ok(Config {
            input_file,
            output_dir,
            execution_path,
        })
    }
}
//Code That can in future be used to make the program more abstract and extensible
/*
pub trait Tree {
    fn has_children(&self) -> Result<usize, &str>;
    fn get_children(&self) -> &Vec<Directory>;
    fn add(&mut self, node: Directory) -> Result<bool, bool>; //maybe change the return type
}
///Object representing a filesystem folder
pub struct Directory {
    pub name: String,
    pub children: Vec<Directory>,
    pub has_children: bool,
}

///Object representing the directory the program was called inside.
///All child directories will be made inside of this directory


impl Tree for Directory {
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

    fn add(&mut self, node: Directory) -> Result<bool, bool> {
        self.children.push(node);
        Ok(true)
    }


}*/
