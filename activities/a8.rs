// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// #[derive(Debug)]
enum Flavor {
    Strawberry,
    Lemon,
    Orange,
}

// #[derive(Debug)]
struct Drink {
    flavor: Flavor,
    ounces: i32,
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Orange,
        ounces: 3
    };
    print_drink(&drink);
}

fn print_drink(drink: &Drink) {
    match drink {
        Drink { flavor: Flavor::Strawberry, ounces } => println!("Flavor: strawberry, ounces: {}", ounces),
        Drink { flavor: Flavor::Lemon, ounces } => println!("Flavor: lemon, ounces: {}", ounces),
        Drink { flavor: Flavor::Orange, ounces } => println!("Flavor: orange, ounces: {}", ounces),
    }
    
    // // deriving Debug
    // match drink {
    //     Drink { flavor, ounces } => println!("{:?}", drink),
    // }
}
