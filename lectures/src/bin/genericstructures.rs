trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T,
}

#[derive(Debug)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32),
}
impl Seat for ConcertSeat {
    fn show(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum AirlineSeat {
    BusinessClass,
    Economy,
    FirstClass,
}
impl Seat for AirlineSeat {
    fn show(&self) {
        println!("{:?}", self);
    }
}

fn ticket_info<T: Seat>(ticket: Ticket<T>) {
    ticket.location.show();
}

fn main() {
    let airline = Ticket {
        location: AirlineSeat::BusinessClass,
    };
    let concert = Ticket {
        location: ConcertSeat::MidSection(662),
    };
    ticket_info(airline);
    ticket_info(concert);
}
