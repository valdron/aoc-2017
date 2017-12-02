use std;
use std::fs::OpenOptions;
use std::io::Read;

pub fn get_input_string(filename: &str) -> std::io::Result<String> {
    let mut res = String::new();
    let mut file = OpenOptions::new().read(true).create(false).open(filename)?;
    file.read_to_string(&mut res)?;
    Ok(res)
}
