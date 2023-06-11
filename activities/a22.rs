// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn clamp_lower() {
        assert_eq!(clamp(10, 100, 1000), 100, "Should be 100")
    }
    #[test]

    fn clamp_upper() {
        assert_eq!(clamp(100, 10, 1000), 100, "Should be 100")
    }

    #[test]
    fn check_div() {
        assert_eq!(div(6, 3), Some(2), "Should be Some(2)");
    }

    #[test]
    fn check_div_zero() {
        assert_eq!(
            div(6, 0),
            None,
            "Cannot divide by zero. Function should return None"
        );
    }

    #[test]
    fn check_concat() {
        assert_eq!(concat("hello", "world"), String::from("helloworld"));
    }
}
