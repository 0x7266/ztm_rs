enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let maybe_user = Some("John");

    match maybe_user {
        Some(name) => println!("{name}"),
        None => println!("no user"),
    }

    if let Some(user) = maybe_user {
        println!("{user}");
    } else {
        println!("no user");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("it's red!");
    } else {
        println!("it's not red");
    }
}
