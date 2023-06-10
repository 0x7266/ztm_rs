fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn check_all_caps() {
        // result, expected, message
        assert_eq!(
            all_caps("test text"),
            String::from("TEST TEXT"),
            "String should be all uppercase"
        );
    }
}
