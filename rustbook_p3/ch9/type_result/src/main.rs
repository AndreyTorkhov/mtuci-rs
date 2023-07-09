// use std::{fs::File, io::ErrorKind};

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },

//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
// }


// fn main() {
//     // use std::fs::File;
//     // use std::io::{self, Read};

//     // use std::fs;
//     // use std::io;

//     // fn read_username_from_file() -> Result<String, io::Error> {
//     //     let username_file_result = File::open("hello.txt");

//     //     let mut username_file = match username_file_result {
//     //         Ok(file) => file,
//     //         Err(error) => return Err(error),
//     //     };

//     //     let mut username = String::new();

//     //     match username_file.read_to_string(&mut username) {
//     //         Ok(_) => Ok(username),
//     //         Err(e) => Err(e),
//     //     }
//     // }

//     // fn read_username_from_file() {
//         // let mut username_file = File::open("hello.txt")?;
//         // let mut username = String::new();
//         // username_file.read_to_string(&mut username)?;
//         // Ok(username);

//         // let mut username = String::new();

//         // File::open("hello.txt")?.read_to_string(&mut username)?;

//         // Ok(username);
//         // fs::read_to_string("hello.txt");

//         // let greeting_file = File::open("hello.txt")?;
//     // }

//     // fn last_char_of_first_line(text: &str) -> Option<char> {
//     //     text.lines().next()?.chars().last()
//     // }

// }


use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>>{
    let greeting_file = File::open("hello.txt");

    Ok(())
}