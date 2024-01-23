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
    let mut logger = utils::logger::ConsoleLogger;
    let first_name = "John";
    let last_name = "Doe";
    print_first_name(first_name, &mut logger);
    print_last_name(last_name, &mut logger);
}

fn print_first_name(first_name: &str, logger: &mut dyn utils::logger::Logger) {
    logger.log(first_name.to_string());
}

fn print_last_name(last_name: &str, logger: &mut dyn utils::logger::Logger) {
    logger.log(last_name.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_first_name_test() {
        let mut test_logger = utils::logger::test::TestLogger::default();
        print_first_name("John", &mut test_logger);
        assert_eq!(String::from("John"), test_logger.0[0]);
        print_last_name("Doe", &mut test_logger);
        assert_eq!(String::from("Doe"), test_logger.0[1]);
    }
}
