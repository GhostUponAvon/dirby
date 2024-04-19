use std::{env, fs, process};
use dirby::Config;
mod file_checks;
mod file_parse;
use file_parse::{Build, DirectoryPaths};
mod directory_construction;

fn main() {
    
    //collect environment arguments
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap();


    //read input file.
    let file_lines: String = fs::read_to_string(config.input_file).expect("Cannot find the specified file. Please check the file name and path.");

    //check the file is valid
    if !file_checks::is_valid_file(&file_lines) {
        process::exit(1);
    }

    
    let file_lines: Vec<String> = file_lines.split('\n').map(|x| x.to_owned()).collect();

    let file_lines = file_lines.iter().map(|x| x.trim().to_owned()).collect();

    let dir_paths: DirectoryPaths = DirectoryPaths::parse(&file_lines, &config.output_dir).unwrap();

    let builder_result = directory_construction::build_fs_tree(dir_paths.get_paths());

    if builder_result.0.is_err() {
        eprintln!("Directory Construction Error: an error occurred while trying to build the directories\nthis likely because the program does not have permission to access the parsed directory,\nor the program attempted to create a folder with an illegal name.\n>Details:\n--->error: {}\n--->occurred when creating: {}", builder_result.0.unwrap_err(), builder_result.1);
    }




}

