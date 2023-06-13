fn main() {
    let mut all_input = Vec::new();
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(err) => println!("Error: {err}"),
        }
    }

    for input in all_input {
        println!("Original: {}\nCapitalized: {}", input, input.to_uppercase());
    }
}

fn get_input() -> std::io::Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
