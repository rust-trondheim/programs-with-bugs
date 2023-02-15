use std::{
    error::Error,
    fs::File,
    io::{self, Read},
    num::ParseIntError,
};

fn read_file(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_number(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>()
}

pub fn process_file_and_parse_number(file_name: &str) -> Result<i32, Box<dyn Error>> {
    let contents = read_file(file_name)?;
    let number = parse_number(&contents)?;
    Ok(number)
}

// #[cfg(test)]
// mod tests {
//     use std::{fs::remove_file, io::Write};

//     use super::*;

//     #[test]
//     fn maperror_test_process_file_and_parse_number_valid_input() {
//         let mut file = File::create("valid_input.txt").unwrap();
//         file.write_all(b"42").unwrap();
//         let result = process_file_and_parse_number("valid_input.txt");
//         remove_file("valid_input.txt").unwrap();
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), 42);
//     }

//     #[test]
//     fn maperror_test_process_file_and_parse_number_nonexistent_file() {
//         let result = process_file_and_parse_number("nonexistent.txt");
//         assert!(result.is_err());
//         match result.err().unwrap() {
//             MyError::Io(_) => assert!(true),
//             _ => assert!(false),
//         }
//     }

//     #[test]
//     fn maperror_test_process_file_and_parse_number_invalid_input() {
//         let mut file = File::create("invalid_input.txt").unwrap();
//         file.write_all(b"not a number").unwrap();
//         let result = process_file_and_parse_number("invalid_input.txt");
//         remove_file("invalid_input.txt").unwrap();
//         assert!(result.is_err());
//         match result.err().unwrap() {
//             MyError::Parse(_) => assert!(true),
//             _ => assert!(false),
//         }
//     }
// }
