# Using Result in Rust

Coming from C#, Java, JavaScript and similar languages, one might be familiar with using `Exception`s (or `Error`s) to signal an error condition by `throw`ing the error. What you'll have to do then (and we're simplifying here for simplicity) is to `catch` the error from the calling method or function.

Rust has the ability to `panic!` (which is Rusts version of `throw`) to signal unhandled errors. But that's not what you want to do most of the time. Most of the time you want to `catch` the error and handle it in the calling function.

To do that in Rust, we have the `Result`-type. When you return the `Result`-type from a function, you declare both what the return-value is if the call to the function succeeded, but also what the possible errors might be if the call fails.

This differs from how the other languages handles "exception" where you cannot explicitly statically type the function or method to tell the caller what errors to expect.

# Bugs

## Bug #1 - basic Result usage

The divider-function (under `src/divider.rs`) will `panic!` and crash the program if you try to divide by 0. Rewrite the function to instead return a `Result`-type. Use the tests in the same file as guideance / check.

<details><summary>🙈 Spoiler alert! A possible solution</summary>

```rust
pub fn divide2(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("Division by zero not allowed!");
    }
    Ok(a / b)
}
```

</details>

## Bug #2 - unwrap all the things!

Developers coming from other languages might call a bunch of functions and methods and wrap them all in a `try`/`catch`. Coming to Rust and having to handle Results will feel unergonomic to some, since Rust hasn o `try`/`catch`-mechanism.

So a typical anti-pattern is either relying on a lot of nested code with `match`es or `if`s:

```rust
fn read_file() -> () {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => println!("File content: {}", content),
        Err(err) => println!("Error reading file: {}", err),
    };
}
```

Or `unwrap`ping the results everywhere to make the code feel more imperative and terse:

```rust
fn read_file() -> () {
    let mut file = File::open("file.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File content: {}", content)
}
```

As we've learned from the first bug though, returning a Result is preferred. And we should not `panic!` on error conditions (like `unwrap` will do) unless strictly necessary.

A very nice mechanism here, is that the Rust syntax allows us to put `?` at the end of calls which returns `Result`s, which will continue the execution if the `Result` is `Ok`, or return from the `Err` to the calling function if it failed.

The read_file-function (under `src/divider.rs`) will `panic!` and if the file doesn't exist or the reafd failed.. Rewrite the function to instead return a `Result`-type, and use the `?` syntax instead of unwrapping or unecessarily nesting with `match`. Use the tests in the same file as guideance / check.

<details><summary>🙈 Spoiler alert! A possible solution</summary>

```rust
pub fn read_file(file_path: &String) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

</details
