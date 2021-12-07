// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print


fn main() {
    // * Use a boolean variable set to the result of
    //   an if..else expression to store whether the value
    let value = 100;
    let is_greater_than_100 = value > 100;
    print_message(is_greater_than_100);
} 

// * Use a function to print the messages
fn print_message(greater_than_100: bool) {
    // * Use a match expression to determine which message
    //   to print
    match greater_than_100 {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}


