// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// * Use a function to add two numbers together
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// * Use a function to display the result
fn display_result(result: i32) {
    // * Use the "{:?}" token in the println macro to display the result
    println!("{:?}", result);
}


fn main() {
    let result = sum(3, 3);
    display_result(result);
}


