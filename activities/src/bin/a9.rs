// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

struct Point(f32, f32);

fn get_tuple() -> Point {
    Point(3.0, 4.0)
}

fn main() {
    let Point(x, y) = get_tuple();
    if y < 5.0 {
        println!("<5");
    } else if y > 5.0 {
        println!(">5");
    } else {
        println!("=5");
    }
}
