
fn get_strings() -> Vec<String> {
    //map over the string slice array to transform items to
    // Strings and convert the array to Vec<String>
    [
        "EEE", "DDD", "", "AAA", "CCC", "", "BBB", "FFF"
    ]
    .map(|a| String::from(a))
    .to_vec()
}

//Borrow an mutable exclusive reference to the string in order
// to change it
fn make_lowercase(str: &mut String) {
    str.to_uppercase();
}

fn main() {
    let vs = get_strings();
    println!("Unsorted strings:");
    //Iterate over a shared reference to vs to avoid moving it
    // into the loop, thus consuming it
    let _vsr = &vs; //rust-analyzer shows the value type of the variable
    for s in &vs {// for iterators, rust-analyzer shows the value type of the items
        println!("{}", s);
    }

    println!("One more time to make sure all the bits are in the right places: {:?}", vs);

    //Filter the empty string items
    let mut vs: Vec<String> = vs.into_iter()
        .filter(|str| !str.is_empty())
        .collect();
    vs.sort();

    println!("Sorted non-empty strings: {:?}", vs);

    fn make_lowercase(s: &mut String) {
        *s = s.to_lowercase()
    }
    for s in vs.iter_mut() {
        make_lowercase(s);
    };
    println!("Lowercased strings: {:?}", vs);
}
