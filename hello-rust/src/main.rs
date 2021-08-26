fn main() {

    let click = MouseClick {x: 100, y: 250};
    println!("Mouse click location: {}, {}", click.x, click.y);
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\n Keys pressed: {}{}", keys.0, keys.1);

    let we_load = WebEvent2::WELoad(true);
    let we_click = WebEvent2::WEClick(click);
    let we_key = WebEvent2::WEKeys(keys);

    println!("\nWebEvent enum structure: \n\n {:#?} \n\n{:#?} \n\n{:#?}", we_load, we_click, we_key)
    
}

// Classic struct with named fields
struct Student {name: String, level: u8, remote: bool}

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;

enum WebEvent {
    // An enum variant can be like a unit struct without fields or data types
    WELoad,
    // An enum variant can be like a tuple struct with data types but no named fields
    WEKeys(String, char),
    // An enu variant can be like a classic struct with named fields and their data types
    WEClick {x: i64, y: i64}
}

// Define a tuple struct
#[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct
#[derive(Debug)]
struct MouseClick {x: i64, y: i64}

// Redefine the enum variants to use the data from th new structs
// Update the page Load variant to have the boolean type
#[derive(Debug)]
enum WebEvent2 {WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress)}