// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let mut logger = utils::logger::ConsoleLogger;
    sum(3, 4, &mut logger);
}

fn sum(a: i32, b: i32, logger: &mut dyn utils::logger::Logger) {
    print_result(a + b, logger);
}

fn print_result(res: i32, logger: &mut dyn utils::logger::Logger) {
    logger.log(res.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut test_logger = utils::logger::test::TestLogger::default();
        sum(10, 20, &mut test_logger);
        assert_eq!(String::from("30"), test_logger.0[0]);
    }
}
