use std::path::Path;

pub fn read_to_chars<T: AsRef<Path>>(pathname: T) -> Vec<char> {
    let data = std::fs::read_to_string(pathname).expect("Unable to open file");
    data.chars()
    
}
