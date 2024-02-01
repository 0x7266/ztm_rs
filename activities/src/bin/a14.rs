// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

impl Person {
    fn new(age: i32, name: &str, favorite_color: &str) -> Self {
        Self {
            age,
            name: name.to_string(),
            favorite_color: favorite_color.to_string(),
        }
    }

    fn print_name(&self) {
        println!("NAME: {}", &self.name);
    }

    fn print_favorite_color(&self) {
        println!("FAVORITE COLOR: {}", &self.favorite_color);
    }
}

fn main() {
    let people = vec![
        Person::new(10, "John", "Red"),
        Person::new(17, "Mary", "Green"),
        Person::new(8, "Peter", "Orange"),
    ];
    for person in people {
        if person.age <= 10 {
            person.print_name();
            person.print_favorite_color();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_person() {
        let p = Person::new(30, "John", "Red");
        assert_eq!(p.favorite_color, "Red");
    }
}
