fn main() {
    let n = 5;

    {
        // shadow variable
        let n = n + 5;

        let n = n * 2;
        
        println!("n = {}", n);
    }

    println!("n = {}", n);
}