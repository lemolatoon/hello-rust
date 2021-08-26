
fn goodbye() {
    println!("Goodbye!");
}

fn main() {
    println!("Hellow, world");
    goodbye();
    is_divisible_by(12, 4);
    is_divisible_by(13, 5);
    is_divisible_by(14, 0);

    if is_zero(0) {
        println!("The value is zero.");
    }
}

fn is_zero(input: u8) -> bool {
    if input == 0 {
        return true;
    }
    false
}

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    // If the divisor is zero, stop execution
    if divisor == 0 {
        println!("\nError! Divison by zero is not allowed.");
        return false;
    } else if dividend % divisor > 0 {
        println!("\n{} % {} has a remainder of {}.", dividend, divisor, (dividend % divisor));
    } else {
        println!("\n{} % {} has no remainder.", dividend, divisor);
    }

    // Create the boolea value and return it to the function caller
    dividend % divisor == 0
}