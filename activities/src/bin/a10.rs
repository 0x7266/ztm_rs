// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let num = 76;
    // let my_bool: bool = if num > 100 { true } else { false };
    let my_bool = num > 100;
    print(&my_bool);
}

fn print(my_bool: &bool) {
    match my_bool {
        true => println!("its big"),
        false => println!("its small"),
    };
}
