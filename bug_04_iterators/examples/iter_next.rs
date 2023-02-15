fn main() {
    let vn = [11,22,33,44];

    //Notice the mutability of the iterator
    let mut vn_iter = vn.iter();

    println!("{:?}", vn_iter.next());
    println!("{:?}", vn_iter.next());
    println!("{:?}", vn_iter.next());
    println!("{:?}", vn_iter.next());
    println!("{:?}", vn_iter.next());

}
