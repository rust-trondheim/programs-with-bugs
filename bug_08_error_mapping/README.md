# Mapping errors

<details><summary>🙈 Spoiler alert! A possible solution</summary>

```rust
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
```

</details
