// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Alcohol {
    Bourbon,
    Beer,
    Wine,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Alcohol,
    fluid_oz: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    match drink.flavor {
        Alcohol::Bourbon => println!("flavor: bourbon"),
        Alcohol::Beer => println!("flavor: beer"),
        Alcohol::Wine => println!("flavor: wine"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}
fn main() {
    let bourbon = Drink {
        flavor: Alcohol::Bourbon, 
        fluid_oz: 2.0,
    };
    print_drink(bourbon);
    let beer = Drink {
        flavor: Alcohol::Beer, 
        fluid_oz: 16.0,
    };
    print_drink(beer);
    let wine = Drink {
        flavor: Alcohol::Wine, 
        fluid_oz: 8.0,
    };
    print_drink(wine);
}
