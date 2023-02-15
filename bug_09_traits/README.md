# Traits

In Rust, a trait defines a set of behaviors that a type can implement. When a type implements a trait, you implement different methods and behaviours defined by the trait. This allows multiple types to share common functionality, without the need for inheritance or interfaces.

Compared to C# which uses inheritance and interfaces, Rust's approach to traits is more flexible and allows for greater code reuse. With inheritance and interfaces, behavior is inherited from a parent class or interface, and is often tied to the class hierarchy.

In Rust, traits can be implemented by any type, regardless of its inheritance or type hierarchy. Which means you can even add traits to types which you haven't defined yourself.lf.

Many inbuilt traits can also leverage mechanisms built into the language.

Take the humble `Debug`-trait. If you implement it for a specific type, then it will be possible to print the contents of an instance of that type:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 25,
        email: String::from("alice@example.com"),
    };

    println!("{:?}", person);
}
```

This will print the content `Person { name: "Alice", age: 25, email: "alice@example.com" }`.

Another trait which is also very useful is the `From`-trait. This trait allows you to define conversions of a value from one type to another type.

In the "error mapping" section we used `map_err` to convert from different `Error`-types to `MyError`. In this section you can instead make the `MyError`-type implement the `From`-trait to define an automatic conversion.

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
