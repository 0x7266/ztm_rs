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
    println!("Full name: {} {}", print_first_name(&first_name), print_last_name(&last_name));
}

fn print_first_name(first_name: &str) -> String {
    println!("{}", first_name);
    String::from(first_name)
}

fn print_last_name(last_name: &str) -> String {
    println!("{}", last_name);
    String::from(last_name)
}


// print_first_name() and print_last_name() return a String. since i'm not storing it in a variable, does it instatly get dropped or does it only get dropped when main() finishes?
