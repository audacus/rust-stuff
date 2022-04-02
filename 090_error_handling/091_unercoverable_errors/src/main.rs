use std::fs::File;
use std::io::ErrorKind;

fn main() {
    {
        let v = vec![1, 2, 3];
        // v[99];
        v[2];
    }

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
        let f = File::open("panic.txt").unwrap();
    }

    {
        // panic!("crash and burn");
    }
}
