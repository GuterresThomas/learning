use std::fs::File;
use std::io::{self, Read};


fn read_username_from_file() -> Result<String, io::Error> {
    /*let username_file_result = File::open("Hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    */

    let mut username_file = File::open("Hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

}

fn main() {

}