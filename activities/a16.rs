// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let john = Student {
        name: "John".to_string(),
        locker: Some(76),
    };
    println!("NAME: {}", john.name);
    // match john.locker {
    //     Some(num) => println!("LOCKER: {}", num),
    //     None => println!("LOCKER: NO LOCKER ASSIGNED"),
    // }
    if let Some(locker) = john.locker {
        println!("LOCKER: {}", locker);
    } else {
        println!("LOCKER: NO LOCKER ASSIGNED")
    }
}
