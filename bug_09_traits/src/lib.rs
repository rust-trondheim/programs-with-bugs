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

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

pub fn process_file_and_parse_number(file_name: &str) -> Result<i32, MyError> {
    let contents = read_file(file_name).map_err(|err| MyError::Io(err))?;
    let number = parse_number(&contents).map_err(|err| MyError::Parse(err))?;
    Ok(number)
}
