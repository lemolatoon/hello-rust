use std;
use std::io;

fn main() {
    let mut str = String::new();

    io::stdin().read_line(&mut str)
        .expect("Failed to read line.");

    println!("str: {}, {:?}", &str, print_typename(&str));

    let num: i32 = str.parse().unwrap();

    println!("num: {}:", num);

    println!("Hello, world!");
}

fn print_typename<T>(_: T) -> &'static str {
    let type_name = std::any::type_name::<T>();
    return type_name;
}