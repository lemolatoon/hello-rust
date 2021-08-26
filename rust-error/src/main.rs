fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("delicious {}", fruit_name),
            None => println!("no fruit :("),
        }
    }

}
