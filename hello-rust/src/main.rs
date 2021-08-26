
fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let (mut index, mut order) = (1, 1);

    let mut car: Car;
    let mut miles = 1000;
    let mut roof = true;
    let mut engine: Transmission; 

    while order <= 11 {
        engine = Transmission::Manual;

        if index % 2 != 0 {
            car = car_factory(colors[index - 1].to_string(), engine, roof, miles)
        } else {
            car = car_factory(colors[index - 1].to_string(), engine, roof, 0)
        }

        println!("{}: {}, Closed roof, {:?}, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1);

        order += 1;
        miles += 1000;

        if index < 4 {
            index += 1;
        } else {
            index = 1;
        }
    }

}

fn car_quality(miles: u32) -> (String, u32) {
    let mut quality: (String, u32) = ("New".to_string(), 0);

    if miles > 0 {
        quality = ("Old".to_string(), miles);
    }

    return quality;
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // TO DO: Replace the "mileage" field from the previous exercise with an "age" field
    // TO DO" The "age" field calls the "car_quality" function with the "miles" input argument 
    let quality = car_quality(miles);
    let age = Age(quality.0, quality.1);
    let car = Car {
        color: color,
        motor: motor,
        roof: roof,
        age: age,
    };

    // Return new instance of "Car" struct
    return car
}


#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: Age,
}

#[derive(PartialEq, Debug)]
struct Age(String, u32);


#[derive(PartialEq, Debug)]
enum Transmission {Manual, SemiAuto, Automatic}
