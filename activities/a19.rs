// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);

    let mut stock_total = 0;
    for (item, item_total) in stock.iter() {
        println!("ITEM: {}", item);
        if *item_total == 0 {
            println!("out of stock\n")
        } else {
            stock_total += item_total;
            println!("ON STOCK: {}\n", item_total);
        }
    }
    println!("\n\nTOTAL ITEMS ON STOCK: {}", stock_total);
}
