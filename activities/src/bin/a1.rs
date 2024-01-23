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
    let first_name = "John";
    let last_name = "Doe";
    print_first_name(first_name);
    print_last_name(last_name);
}

fn print_first_name(first_name: &str, logger: &mut dyn Logger) {
    logger.log(first_name.to_string());
}

fn print_last_name(last_name: &str, logger: &mut dyn Logger) {
    logger.log(last_name.to_string());
}
