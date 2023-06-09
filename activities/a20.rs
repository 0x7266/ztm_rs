// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(command: &str) -> Option<Self> {
        match command.to_lowercase().trim() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_state(state: PowerState) {
    match state {
        PowerState::Off => println!("Turning off"),
        PowerState::Sleep => println!("Sleeping"),
        PowerState::Reboot => println!("Rebooting"),
        PowerState::Shutdown => println!("Shutting down"),
        PowerState::Hibernate => println!("Hibernating"),
    }
}

fn main() {
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => match PowerState::new(&buffer) {
            Some(state) => print_state(state),
            None => println!("Not a valid command"),
        },
        Err(_) => println!("Something went wrong while reading user input"),
    };
}

// ---------------------------------------------------------------------

// fn main() -> std::io::Result<()> {
//     let mut buffer = String::new();
//     std::io::stdin().read_line(&mut buffer)?;
//     print_state(convert_input(buffer));
//     Ok(())
// }

// fn print_state(state: Result<PowerState, String>) {
//     match state {
//         Ok(PowerState::Off) => println!("Off"),
//         Ok(PowerState::Sleep) => println!("Sleep"),
//         Ok(PowerState::Reboot) => println!("Reboot"),
//         Ok(PowerState::Shutdown) => println!("Shutdown"),
//         Ok(PowerState::Hibernate) => println!("Hibernate"),
//         Err(msg) => println!("{msg}"),
//     }
// }

// fn convert_input(arg: String) -> Result<PowerState, String> {
//     match arg.to_lowercase().trim() {
//         "off" => Ok(PowerState::Off),
//         "sleep" => Ok(PowerState::Sleep),
//         "reboot" => Ok(PowerState::Reboot),
//         "shutdown" => Ok(PowerState::Shutdown),
//         "hibernate" => Ok(PowerState::Hibernate),
//         _ => Err(String::from("Not a valid command")),
//     }
// }
