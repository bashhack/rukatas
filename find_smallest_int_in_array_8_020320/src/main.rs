// Link:
// https://www.codewars.com/kata/55a2d7ebe362935a210000b2

// Description:
// Given an array of integers your solution should find the smallest integer.
// For example:
//     Given [34, 15, 88, 2] your solution will return 2
//     Given [34, -345, -1, 100] your solution will return -345
// You can assume, for the purpose of this kata, that the supplied array will not be empty.

// fn find_smallest_int(arr: &[i32]) -> i32 {
//   // your code here
// }

fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
        assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
    }
}
