trait Clicky {
    fn click(&self);
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) {
        println!("click clack");
    }
}

struct Mouse;

impl Clicky for Mouse {
    fn click(&self) {
        println!("click");
    }
}

fn borrow_clicky_1(obj: &dyn Clicky) {
    obj.click();
}

fn borrow_clicky_2(obj: Box<dyn Clicky>) {
    obj.click();
}

// fn borrow_clicky_3(obj: Box<dyn Clicky>) {
//     print!("FROM THE VEC: ");
//     obj.click();
// }

fn make_clicks(clickeys: Vec<Box<dyn Clicky>>) {
    for clicker in clickeys {
        print!("FROM THE VEC: ");
        clicker.click();
    }
}

fn main() {
    let keeb = Keyboard;
    borrow_clicky_1(&keeb);
    borrow_clicky_1(&Keyboard);

    let keeb = Box::new(Keyboard);
    borrow_clicky_2(keeb);
    borrow_clicky_2(Box::new(Keyboard));

    // let keeb: Box<dyn Clicky> = Box::new(Keyboard);
    // let mouse: Box<dyn Clicky> = Box::new(Mouse);
    // let clickers = vec![keeb, mouse];
    let keeb = Box::new(Keyboard);
    let mouse = Box::new(Mouse);
    let clickers: Vec<Box<dyn Clicky>> = vec![keeb, mouse];
    // for clicker in clickers {
    //     borrow_clicky_3(clicker);
    // }
    make_clicks(clickers);
}
