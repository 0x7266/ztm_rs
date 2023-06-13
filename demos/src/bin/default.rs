#[derive(Debug)]
struct Package {
    weight: f32,
}

impl Package {
    fn new(weight: f32) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

fn main() {
    let package = Package::default();
    println!("{package:?}");
}
