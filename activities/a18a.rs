// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: i32,
}

fn main() {
    let john = Customer {
        name: "John".to_owned(),
        age: 11,
    };
    match check_age(john) {
        Ok(_) => println!("Customer can make restricted purchase"),
        Err(msg) => println!("{msg}"),
    }
}

fn check_age(customer: Customer) -> Result<(), String> {
    if customer.age >= 21 {
        return Ok(());
    }
    Err(format!("Customer can't make restricted purschase! Customer is {} years old, but must be at least 21 to make a restricted purchase", customer.age))
}
