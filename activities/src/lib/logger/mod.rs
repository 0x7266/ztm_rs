pub trait Logger {
    fn log(&mut self, value: String);
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, value: String) {
        println!("{}", value);
    }
}

pub mod test {
    use super::*;

    #[derive(Default)]
    pub struct TestLogger(pub Vec<String>);

    impl Logger for TestLogger {
        fn log(&mut self, value: String) {
            self.0.push(value);
        }
    }
}
