fn main() {
    println!("Hello, world!");

    let a_number = 10;
    let mut b = 32;

    println!("b = {}", b);

    b = 15;

    println!("b = {}", b);

    println!("it is like .format {} + {} = {}", a_number, b, a_number + b);
}