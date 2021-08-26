fn main() {
    // Specify the data type "char"
    let char_1: char = 'S';
    let char_2: char = 'f';
    // one quote

    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = "☺";

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";


    // Specify the data type "str" with the reference synta "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, char_1, string_1, char_2, string_2)
}