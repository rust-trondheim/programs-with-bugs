# Mapping errors

Rust offers strong support for error handling. In the case of "bug 07," we were fortunate in that the error returned was of the same type. But what if we encounter different types of errors? In such cases, we need to consolidate them into a single error-type to handle them when returning a Result.

One simple way to do this is to use the `Box`-type to encapsulate different error-values. However, this means that we can only evaluate the possible error-types at runtime. To be more explicit about what our possible errors can be, we should define our own `enum` or `Error`-type.

With our own `enum`, we can map from the possible error cases to our defined MyError using the `map_err`-function on returned Results. This way, we can handle different types of errors more effectively and explicit.

Take the code in `src/lib.rs` which uses the `Box`-type, and instead define a `MyError`-type which is returned from `process_file_and_parse_number`. Map the `Result`s returned from `read_file` and `parse_number` over to MyError with `map_err`.

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
