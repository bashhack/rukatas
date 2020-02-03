use std::ops::Mul;

// Link:
https://www.codewars.com/kata/50654ddff44f800200000004

// Description:
// The code does not execute properly. Try to figure out why.

// fn multiply(a: i32, b: i32) {
//   a * b
// }

// fn multiply(a: i32, b: i32) -> i32 {
//     a * b
// }

// Alternatively:
fn multiply<T: Mul>(a: T, b: T) -> T::Output {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(multiply(3, 5), 15)
    }
}
