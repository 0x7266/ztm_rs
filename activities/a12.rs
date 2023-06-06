// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Black,
    Brown,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("COLOR: black"),
            Color::Brown => println!("COLOR: brown"),
            Color::White => println!("COLOR: white"),
        }
    }
}

struct Dimensions {
    width: f32,
    height: f32,
    depth: f32,
}

impl Dimensions {
    fn print(&self) {
        println!("WIDTH: {}", self.width);
        println!("HEIGHT: {}", self.height);
        println!("DEPTH: {}", self.depth);
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f32,
    color: Color,
}

impl Box {
    fn new(dimensions: Dimensions, weight: f32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        &self.dimensions.print();
        println!("WEIGHT: {}", &self.weight);
        &self.color.print();
    }
}

fn main() {
    let my_box = Box::new(
        Dimensions {
            width: 3.0,
            height: 4.0,
            depth: 3.0,
        },
        3.3,
        Color::Black,
    );
    my_box.print_characteristics();
}
