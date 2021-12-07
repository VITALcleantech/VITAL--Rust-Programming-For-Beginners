// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


enum Color {
    // * Use an enum with color names as variants
    Red, 
    Green, 
    Blue,
}
// * Use a function to print the color name
fn print_color(my_color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match my_color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    print_color(Color::Blue);
}