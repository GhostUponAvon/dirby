#[cfg(target_os = "linux")]
const INVALID_FS_CHARS: &str = "/"; //characters forbidden in a linux folder or file name and for this program.

#[cfg(target_os = "windows")]
const INVALID_FS_CHARS: &str = "<>:\"/\\|?*"; //characters forbidden in a windows folder or file name and for this program.

#[cfg(target_os = "macos")]
const INVALID_FS_CHARS: &str = ":/"; //characters forbidden in a macos folder or file name and for this program.


pub fn is_valid_file(file: &String) -> bool{

    //check that the file is valid
    if file.contains(|x| is_valid_fs_char(&x)) {
        return false
    };
    true
}

    

fn is_valid_fs_char(character: &char) -> bool {
    
    if INVALID_FS_CHARS.contains(*character) {
        return true
    }
    return false

}

//NOTE: add option to log any errors found in the file to a file.

