pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero not allowed!")
    }
    a / b
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn divider_valid_args_should_return_ok_result() {
//         assert_eq!(divide(10, 2), Ok(5));
//     }

//     #[test]
//     fn divider_divide_by_zero_should_return_err() {
//         assert_eq!(divide(10, 0), Err("Division by zero not allowed!"));
//     }
// }
