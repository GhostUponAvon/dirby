use std::{env, fs};

pub struct Config {
    pub execution_path: String,
    pub input_file: String,
    pub ouput_dir: String,
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        let input_file: String = args[1].clone();
        let output_dir: String = args[2].clone();
        
        //FILL
    }
} 