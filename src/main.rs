use std::{env, fs, process};
use dirby::Config;
use file_parse::{Build, DirectoryPaths};
mod file_checks;
mod file_parse;
mod directory_construction;

fn main() {
    
    //collect enviroment arguments
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap();


    //read input file.
    let file_lines: String = fs::read_to_string(config.input_file).expect("Cannot find the specified file. Please check the file name and path.");

    /*
    if !file_checks::is_valid_file(&file_lines) {
        process::exit(1);
    }*/

    
    let file_lines: Vec<String> = file_lines.split('\n').map(|x| x.to_owned()).collect();

    let file_lines = file_lines.iter().map(|x| x.trim().to_owned()).collect();

    let dir_paths: DirectoryPaths = DirectoryPaths::parse(&file_lines, &config.ouput_dir).unwrap();

    /*
    for x in dir_paths.get_paths() {
        println!("{}", x.to_str().unwrap());
    } */




}
/*
fn is_valid_file(character: &char) -> bool{

    if INVALID_FS_CHARS.contains(*character) {
        return false
    }
    return true
}
*/
