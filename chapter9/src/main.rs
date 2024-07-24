use std::fs::File;
use std::io::{self, Read};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {:?}", error),
    };

    match read_username() {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error reading username: {:?}", error),
    }
}

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
