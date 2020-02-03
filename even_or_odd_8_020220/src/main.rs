// Link;
// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust

// Description:
// Create a function (or write a script in Shell) that takes an integer as an argument and returns "Even" for even numbers or "Odd" for odd numbers.

// fn even_or_odd(i: i32) -> &'static str {}

fn even_or_odd(i: i32) -> &'static str {
    match i % 2 {
        0 => "Even",
        _ => "Odd",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(even_or_odd(0), "Even");
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(1), "Odd");
        assert_eq!(even_or_odd(7), "Odd");
        assert_eq!(even_or_odd(-1), "Odd");
    }
}
