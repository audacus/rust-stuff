use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    {
        let f = match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(error) => panic!("Problem creating the file {:?}", error),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }

    {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    {
        let file_name = "hello.txt";
        let error_message = &format!("Could not read username from '{}'", file_name).to_string();

        // read username from file function
        let username = read_username_from_file(file_name).expect(error_message);

        // read username from file short
        let username = fs::read_to_string(file_name).expect(error_message);

        println!("username: {}", username);
    }

    {
        let f = File::open("hello.txt")?; // -> function needs to return Result or Option
    }

    {
        let last = last_char_of_first_line("\nblabla").expect("could not get last char of first line");
        println!("last char of first line: {}", last);

    }

    {
        let f = File::open("expect.txt").expect("Failed to open 'expect.txt'");
        let f = File::open("unwrap.txt").unwrap();
    }

    {
        // panic!("crash and burn");
    }
    Ok(())
}

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
