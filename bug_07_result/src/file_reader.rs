use std::{
    fs::{self, File},
    io::{Read, Write},
};

pub fn read_file(file_path: &String) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[test]
fn test_read_file_with_existing_file() {
    let file_path = String::from("file.txt");

    // create a file with some content
    let mut file = File::create(&file_path).unwrap();
    let contents = "Hello, World!";
    file.write_all(contents.as_bytes()).unwrap();

    // read the contents of the file
    let result = read_file(&file_path).unwrap();

    // check if the contents of the file are as expected
    assert_eq!(result, contents);

    // remove the file
    fs::remove_file(&file_path).unwrap();
}

#[test]
fn test_read_file_with_nonexistent_file() {
    let file_path = String::from("nonexistent.txt");

    // remove the file if it exists
    fs::remove_file(&file_path).ok();

    // try to read from a nonexistent file
    let result = read_file(&file_path);

    // check if an error is returned
    assert!(result.is_err());
}
