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
       let mut current_path: PathBuf = PathBuf::new();
       let mut current_depth: usize = 0;

       for line in file {
           //check depth of the line!()
           //
           //determine based on depth to move forwards/hold/backwards
           //
           //

           //forward
           {
               current_path.push(/*the unadulterated name of the file path element*/);
               //determine if anything else needs to happen
           }

           //hold
           {
               paths.paths.push(current_path);
               current_path.pop();
               current_path.push(/*the unadulterated name of the file path element*/)
           }

           //backwards
           {
               //push the current file path to the paths variable
               //
               //determine using the current depth how many times to pop the current_path variable
               //
               //push the line variable to the current_path variable
           }
       }

        /* 
        let mut paths: DirectoryPaths = DirectoryPaths::new(file.len());
        let mut line_window: Vec<String> = vec![root_dir.to_owned()];

        let mut current_depth: usize = 0;

        for line in file { 
            //println!("{}", line);
            //forward
            
            if line[current_depth..].contains("/") {
                println!("{} was -> forward", line);
                current_depth += 1;
                let new_node: String = line[current_depth..].to_string();
                //current_node.has_children = true;
                //current_node.add(new_node);
                line_window.push(new_node);
            } 

            //hold
            else if !line[current_depth..].contains("/") {
                println!("{} was -> hold", &line);
                let mut path: PathBuf = PathBuf::new();

                if false/*line_window.len() == 1*/ {
                    path.push(&line_window[0].to_owned());
                    path.push(&line[current_depth..].to_string());
                    paths.paths.push(path);
                } else {
                    for dir in &line_window {
                        path.push(dir.to_owned());
                    
                        
                        //if the Vector is more than one entry long go through it recursivly.
                    }
                    path.push(&line[current_depth..].to_string());
                    paths.paths.push(path);
                }
            }

            //backwards
            else if line.contains("/") {
                println!("{} was -> backwards", line);

                let mut path: PathBuf = PathBuf::new();
                
                
                let mut reverse_depth: usize = 0;
                
                //determine new depth
                for char in line.chars() {
                    if char == '/' {
                        reverse_depth += 1;
                    }
                }

                
                //create entry for previous line
                for dir in &line_window {
                    path.push(dir.to_owned());
                
                }
                path.push(line[reverse_depth..].to_string());
                paths.paths.push(path);



                //remove old names from line window since we are reversing our depth
                for _i in 0..current_depth-reverse_depth {
                    line_window.pop();
                }

                // set new depth values
                current_depth = current_depth-reverse_depth

            } else {
                return Err("failed to parse file")
            }

            //add handler for if the next line does not move forward but stays put

            //add handler to generate path variable entry and move backwards in the lline windows variable when the next lien moves backwards.
            
            //continue here and add handler for when there isn't a new sub node

            
            //delegate job to a function responsible for building the directory tree.
        }
        
    dbg!(&paths);*/
    Ok(paths)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_parse_expected_output() {
        let root = "test";
        let file = vec!["file".to_owned(), "file 2".to_owned(), "/file 3".to_owned(), "//file 4".to_owned(), "file 5".to_owned()];

        let var = DirectoryPaths::parse(&file, root).unwrap();

        for x in var.get_paths() {
            println!("{}", x.to_str().unwrap());
        }
        assert!(true);
    }
}
