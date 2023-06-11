// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f32, String),
    Vip(f32, String),
    Standard(f32),
}

fn main() {
    let b_ticket = Ticket::Backstage(200.0, String::from("John"));
    let v_ticket = Ticket::Vip(150.0, String::from("Mary"));
    let s_ticket = Ticket::Standard(80.0);

    let tickets = vec![b_ticket, v_ticket, s_ticket];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("PRICE: {}\nHOLDER: {}\n\n", price, holder)
            }
            Ticket::Vip(price, holder) => println!("PRICE: {}\nHOLDER: {}\n\n", price, holder),
            Ticket::Standard(price) => println!("PRICE: {}\n\n", price),
        }
    }
}
