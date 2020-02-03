// Link:
// https://www.codewars.com/kata/56dec885c54a926dcd001095/train/rust

// Description:
// Very simple, given a number, find its opposite.
// Examples:
// 1: -1
// 14: -14
// -34: 34

// fn opposite(number: i32) -> i32 {}

fn opposite(number: i32) -> i32 {
    -number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(opposite(1), -1);
        assert_eq!(opposite(-1), 1);
    }
}
