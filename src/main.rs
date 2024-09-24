use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_user_from_file_v3(path: &str) -> Result<String, io::Error> { 
    fs::read_to_string(path) // utilize fs standard library
}

// ? operator to return Error to Result or Option type automatically by from method in From trait
fn read_user_from_file_v2(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_user_from_file(path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };  

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn main() {
    let path = String::from("hello.txt");

    let greeting_file_result = File::open(path.as_str());

    let greeting = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path.as_str()) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // using unwrap_or_else method
    let greeting2 = File::open(path.as_str()).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => File::create(path.as_str()).unwrap_or_else(|error| {
            panic!("Problem creating the file: {error:?}");
        }),
        other_error => panic!("Problem opening the file: {other_error:?}"),
    });

    File::open(path.as_str()).unwrap(); // this function maybe panic
}
