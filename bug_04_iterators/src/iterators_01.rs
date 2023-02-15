fn get_strings() -> Vec<String> {
    [
        "EEE", "DDD", "", "AAA", "CCC", "", "BBB", "FFF"
    ]
}

fn main() {
    let mut vs = get_strings();
    println!("Unsorted strings:");
    for s in vs {
        println!("{}", s);
    }
    println!("One more time to make sure all the bits are in the right places: {:?}", vs);

    println!("Sorted non-empty strings:");

    //Filter the empty string items
    vs = vs.iter()
        .filter(|s| !s.is_empty())
        .sort();
    println!("{:?}", vs);

    fn make_lowercase(str: String) {
        str.to_uppercase();
    }
    for (index, s) in vs.into_iter().enumerate() {
        vs[index] = make_lowercase(s.clone());
    };
    println!("Lowercased strings: {:?}", vs);
}
