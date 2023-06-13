trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Checked in as pilot");
    }

    fn process(&self) {
        println!("Pilot enters the cockpit");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Checked in as passenger");
    }

    fn process(&self) {
        println!("Passenger takes a seat");
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("Cargo checked in");
    }

    fn process(&self) {
        println!("Cargo moved to storage");
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

fn main() {
    let john = Passenger;
    let mary = Pilot;
    let cargo_1 = Cargo;
    let cargo_2 = Cargo;
    process_item(mary);
    process_item(john);
    process_item(cargo_1);
    process_item(cargo_2);
}
