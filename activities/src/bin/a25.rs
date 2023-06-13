// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Trig {
    fn calc_perimeter(&self) -> f32;
}

struct Square {
    side: f32,
}
impl Trig for Square {
    fn calc_perimeter(&self) -> f32 {
        self.side * 4.0
    }
}

struct Triangle {
    side: (f32, f32, f32),
}
impl Trig for Triangle {
    fn calc_perimeter(&self) -> f32 {
        self.side.0 + self.side.1 + self.side.2
    }
}

fn print_perimeter(shape: impl Trig) {
    println!("{}", shape.calc_perimeter());
}

fn main() {
    print_perimeter(Square { side: 3.0 });
    print_perimeter(Triangle {
        side: (3.0, 4.0, 7.0),
    });
}
