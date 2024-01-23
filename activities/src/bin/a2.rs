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
    let mut logger = ConsoleLogger;
    sum(3, 4, &mut logger);
}

fn sum(a: i32, b: i32, logger: &mut dyn Logger) {
    print_result(a + b, logger);
}

fn print_result(res: i32, logger: &mut dyn Logger) {
    logger.log(res.to_string());
}

trait Logger {
    fn log(&mut self, value: String);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, value: String) {
        println!("{}", value);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestLogger(Vec<String>);

    impl Logger for TestLogger {
        fn log(&mut self, value: String) {
            self.0.push(value)
        }
    }

    #[test]
    fn test_sum() {
        let mut test_logger = TestLogger::default();
        sum(10, 20, &mut test_logger);
        assert_eq!(String::from("30"), test_logger.0[0]);
    }
}
