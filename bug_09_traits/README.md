# Traits

Something something can actually remove manual mapping. Traits woohoo.

<details><summary>🙈 Spoiler alert! A possible solution</summary>

```rust
#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}


impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

pub fn process_file_and_parse_number(file_name: &str) -> Result<i32, MyError> {
    let contents = read_file(file_name)?;
    let number = parse_number(&contents)?;
    Ok(number)
}
```

</details
