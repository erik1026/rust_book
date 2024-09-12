use std::fs::{self, File};
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// More simple than match
fn read_uersname_from_file_simple() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username);
    Ok(username)
}

// More simple version
fn read_username_from_file_more_simple() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username);

    Ok(username)
}

// Most simple version
fn read_usename_from_file_most_simple() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// '?' can also be used with Option<T> types

fn main() {}
