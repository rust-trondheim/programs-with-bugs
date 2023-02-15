# Iterators

## My story
So I am a TypeScript developer.
I am happy.
I can create objects according to interfaces with members of various--but specific--types and iterate over them with ease.
If am dutifully declaring the types on my variable and functions, and I feel secure whenever I push to prod.
Life is good!

Now, I'm working on a Rust project where I am rewriting my `iterator_01.ts` file.
The program doing the following:
Gets a list of strings from a function and prints them sequentially.

Removes all the empty strings and prints them in lexically sorted order.

At the end it mutates all the string items into lowercase strings and prints the resulting array.

## Running the programs and examples
You can work with the file `src/iterators_01.rs`.

To run the buggy program that you eventually will fix: `cargo run --bin iterators_01`

To run the bugfix suggestion program: `cargo run --bin iterators_01`

To run the `iter_next` example: `cargo run --example iter_next`

