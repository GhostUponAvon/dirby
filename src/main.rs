use std::{env, fs, os::windows::process, path::Path};
use dirby::Config;

#[cfg(target_os = "linux")]
const INVALID_FS_CHARS: String = String::from("/"); //characters forbidden in a linux folder or file name and for this program.

#[cfg(target_os = "windows")]
const INVALID_FS_CHARS: String = String::from("<>:\"/\\|?*"); //characters forbidden in a windows folder or file name and for this program.

#[cfg(target_os = "macos")]
const INVALID_FS_CHARS: String = String::from(":/"); //characters forbidden in a macos folder or file name and for this program.


fn main() {
    
    //collect enviroment arguments
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap();


    //read input file.
    let files_lines: String = fs::read_to_string(config.input_file).expect("Cannot find the specified file. Please check the file name and path.");
    
    //check that the file is valid
    if files_lines.contains(|x| is_valid_fs_character(&x)) {
        //Exit the procces here
    };


    let file_lines: Vec<String> = files_lines.split('\n').map(|x| x.to_owned()).collect();

    




}

fn is_valid_fs_character(character: &char) -> bool{

    //perform checks on each characters
}
