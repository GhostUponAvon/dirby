use std::{env, fs, process, path::Path};
use dirby::Config;
mod file_checks;


fn main() {
    
    //collect enviroment arguments
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap();


    //read input file.
    let files_lines: String = fs::read_to_string(config.input_file).expect("Cannot find the specified file. Please check the file name and path.");
    

    if !file_checks::is_valid_file(&files_lines) {
        process::exit(1);
    }

    
    let file_lines: Vec<String> = files_lines.split('\n').map(|x| x.to_owned()).collect();

    




}
/*
fn is_valid_file(character: &char) -> bool{

    if INVALID_FS_CHARS.contains(*character) {
        return false
    }
    return true
}
*/
