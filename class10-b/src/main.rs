use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Error, Read, Write};
fn main() {
    file_check_create();
    match read_file() {
        Ok(string) => println!("String {}", string),
        Err(e) => panic!("Error in File ,{:?}", e),
    }
}

fn read_file() -> Result<String, io::Error> {
    let student = File::open("hello.txt");

    let mut task = match student {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(mut f) => {
                    f.write(b"Hello All Students")?;
                    f
                }
                Err(e) => return Err(e),
            },
            _ => return Err(error),
        },
    };
    let mut data = String::new();

    task.read_to_string(&mut data);

    Ok(data)
}

fn file_check_create() {
    let student = File::open("hello.txt");
    match student {
        Ok(file) => println!("File Found {:#?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(mut f) => {
                    f.write(b"Hello All Students");
                    println!("File Created")
                }
                Err(e) => panic!("Error ,{:?}", e),
            },
            _ => panic!("Error reading File"),
        },
    }
}
