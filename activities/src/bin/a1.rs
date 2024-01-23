// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let mut logger = ConsoleLogger;
    let first_name = "John";
    let last_name = "Doe";
    print_first_name(first_name, &mut logger);
    print_last_name(last_name, &mut logger);
}

fn print_first_name(first_name: &str, logger: &mut dyn Logger) {
    logger.log(first_name.to_string());
}

fn print_last_name(last_name: &str, logger: &mut dyn Logger) {
    logger.log(last_name.to_string());
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
            self.0.push(value);
        }
    }

    #[test]
    fn print_first_name_test() {
        let mut test_logger = TestLogger::default();
        print_first_name("John", &mut test_logger);
        assert_eq!(String::from("John"), test_logger.0[0]);
        print_last_name("Doe", &mut test_logger);
        assert_eq!(String::from("Doe"), test_logger.0[1]);
    }
}
