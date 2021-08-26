
fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, conertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, conertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 2 = {}, {:?} transmission, conertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car = Car {color: color, transmission: transmission, convertible: convertible, mileage: 0};

    car
}

// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
// TO DO: Fix enum definition so code compiles
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}
