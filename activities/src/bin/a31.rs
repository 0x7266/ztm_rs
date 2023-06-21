// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Cost {
    fn get_cost(&self) -> f64;
}

struct Carpet(f64);
impl Cost for Carpet {
    fn get_cost(&self) -> f64 {
        self.0 * 10.
    }
}
struct Tile(f64);
impl Cost for Tile {
    fn get_cost(&self) -> f64 {
        self.0 * 15.
    }
}
struct Wood(f64);
impl Cost for Wood {
    fn get_cost(&self) -> f64 {
        self.0 * 20.
    }
}

fn calculate_cost(order: Vec<Box<dyn Cost>>) -> f64 {
    order.iter().map(|product| product.get_cost()).sum()
}

fn main() {
    let carpet = Box::new(Carpet(40.));
    let tile = Box::new(Tile(60.));
    let wood = Box::new(Wood(100.));

    let order: Vec<Box<dyn Cost>> = vec![carpet, tile, wood];

    println!("{}", calculate_cost(order));
}

// PLAYING AROUND WITH ENUMS
// trait Cost {
//     fn get_cost(&self) -> f64;
// }

// enum Material {
//     Carpet(f64),
//     Tile(f64),
//     Wood(f64),
// }

// impl Cost for Material {
//     fn get_cost(&self) -> f64 {
//         match self {
//             Self::Carpet(square_meters) => 10. * square_meters,
//             Self::Tile(square_meters) => 15. * square_meters,
//             Self::Wood(square_meters) => 20. * square_meters,
//         }
//     }
// }

// fn calculate_cost(order: Vec<Box<dyn Cost>>) -> f64 {
//     order.iter().map(|product| product.get_cost()).sum()
// }
// // fn calculate_cost(order: Vec<Box<Material>>) -> f64 {
// //     order.iter().map(|product| product.get_cost()).sum()
// // }
// // fn calculate_cost(order: Vec<Material>) -> f64 {
// //     order.iter().map(|product| product.get_cost()).sum()
// // }

// fn main() {
//     let carpet = Box::new(Material::Carpet(40.));
//     let tile = Box::new(Material::Tile(60.));
//     let wood = Box::new(Material::Wood(100.));

//     let order: Vec<Box<dyn Cost>> = vec![carpet, tile, wood];
//     // let order: Vec<Material> = vec![*carpet, *tile, *wood];
//     // let order: Vec<Box<Material>> = vec![carpet, tile, wood];

//     println!("{}", calculate_cost(order));
// }
