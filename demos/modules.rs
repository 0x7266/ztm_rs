use std::collections::HashMap;
mod greet {
    // if HashMap is going to be used within greet module, greet module must "use" because each module has it own version of different use statements
    use std::collections::HashMap;
    fn hello() {
        println!("hello");
    }

    fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    // use greet::*;
    use greet::hello;
    hello();
    greet::goodbye();
    math::add(1, 1);
}
