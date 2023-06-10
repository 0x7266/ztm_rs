fn main() {
    let mut data = Some(10);
    while let Some(i) = data {
        println!("{i}");
        data = None;
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let mut numbers_iter = numbers.iter();
    while let Some(num) = numbers_iter.next() {
        println!("{num}");
    }
}
