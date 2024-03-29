// Topic: Generics & Functions
//
// Requirements:
// * Create a function that accepts the Priority trait as a generic parameter
//   * The function should print out the guest and their priority
// * Use the function with at least two different guests
//
// Notes:
// * Use the debug token "{:?}" to print out the information
// * Use the compiler to guide you to the correct generic constraints needed

#[derive(Debug, PartialEq)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug, PartialEq)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

// syntax options:

// fn print_guest_and_priority<T: Priority>(guest: T) {
//     println!("{:?}", guest.get_priority());
// }

// --------------- or ---------------
fn print_guest_and_priority<T>(guest: T)
where
    T: Priority,
{
    println!("{:?}", guest.get_priority());
}

// --------------- or ---------------
// fn print_guest_and_priority(guest: impl Priority) {
//     println!("{:?}", guest.get_priority());
// }

fn main() {
    let guest = Guest;
    print_guest_and_priority(guest);
    print_guest_and_priority(ImportantGuest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {
        let guest = Guest;
        assert_eq!(ServicePriority::Standard, guest.get_priority());
    }
}
