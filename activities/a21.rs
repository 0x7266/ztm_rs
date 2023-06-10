// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    // decided to create a vec to test all possibilities at once
    let names = vec!["katie", "sam", "john", "matt"];
    for name in names {
        let user = find_user(name).map(|user_id| User {
            user_id,
            name: name.to_string(),
        });
        match user {
            Some(user) => println!("{user:?}"),
            None => println!("not found"),
        }
        // -----------------------------------------------
        // if let Some(user_id) = find_user(name) {
        //     println!(
        //         "{:?}",
        //         User {
        //             user_id,
        //             name: name.to_string()
        //         }
        //     );
        // } else {
        //     println!("not found")
        // }
        // -----------------------------------------------
        // match find_user(name) {
        //     Some(user_id) => {
        //         println!(
        //             "{:?}",
        //             User {
        //                 user_id,
        //                 name: name.to_string()
        //             }
        //         );
        //     }
        //     None => println!("not found"),
        // }
    }
}
