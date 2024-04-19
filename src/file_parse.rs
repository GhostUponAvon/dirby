use std::{path::PathBuf, usize};


pub trait Build {
    fn new(size: usize) -> DirectoryPaths;
    fn parse<'a>(file: &'a Vec<String>, root_dir: &'a str) -> Result<DirectoryPaths, &'a str>;
    fn get_paths(&self) -> Vec<PathBuf>;

    //may add functionality to return it as a Vector of type String.
}
#[derive(Debug)]
pub struct DirectoryPaths {
    paths: Vec<PathBuf>
}

impl Build for DirectoryPaths {
    
    fn new(size: usize) -> DirectoryPaths {
        DirectoryPaths {paths: Vec::with_capacity(size)}
    }

    fn get_paths(&self) -> Vec<PathBuf> {
        return self.paths.clone()
    }

    fn parse<'a>(file: &'a Vec<String>, root_dir: &'a str) -> Result<DirectoryPaths, &'a str> {
       let mut paths: DirectoryPaths = DirectoryPaths::new(file.len());
       let mut current_path: PathBuf = PathBuf::new(); current_path.push(root_dir.to_owned());
       let mut current_depth: isize = -1;

       //remove the last element of the file variable and render file variable immutable.
       let mut file = file.clone(); file.pop(); let file = file;


       for line in file {
           //check depth of the line
           //
           //determine based on depth to move forwards/hold/backwards
           //
           let mut new_depth: isize = 0;
           for chr in line.chars() {
               if chr == '/' {
                   new_depth += 1;
               }else {
                   break;
               }
           }

           //forward
           if new_depth > current_depth {
               current_path.push(line[new_depth as usize..].to_owned());
           }

           //hold
           else if current_depth == new_depth {
               paths.paths.push(current_path.clone());
               current_path.pop();
               current_path.push(line[new_depth as usize..].to_owned());
           }

           //backwards
           else if new_depth < current_depth {
               //push the current file path to the paths variable
               paths.paths.push(current_path.clone());

               //determine using the current depth how many times to pop the current_path variable
               for _ in 0..(current_depth-new_depth+1) {
                   current_path.pop();
               }
               current_path.push(line[new_depth as usize..].to_owned());
           }
           current_depth = new_depth;
       }
        paths.paths.push(current_path.clone());

    Ok(paths)
    }
}
