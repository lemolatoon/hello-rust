
fn main() {
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    let zeros = vec![0; 5];
    println!("Zeros: {:?}", zeros);

    let mut fruit = Vec::new();

    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    fruit.push("Cherry");
    fruit[2] = "Pineapple";
    println!("Fruits: {:?}, three = {}", fruit, fruit[2])
}
