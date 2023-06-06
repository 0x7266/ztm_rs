// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    const text: &str = "ThIs Is A tEsT tExT";
    println!("{}", text);
    let uppercase_text = text.to_uppercase();
    println!("{}", lowercase_text);
    let lowercase_text = uppercase_text.to_lowercase();
    println!("{}", text.to_uppercase());
}
