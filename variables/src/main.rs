//Using directives globally
#![warn(unused_variables)]
#![allow(unused_variables)]

//Constants - can be declared in the file level and are immutable. A value must be assigned by compile time and an explicit data type assigned
const TAX_RATE: f64 = 30.5;
const TOUCHDOWN_POINTS: i32 = 6;


fn assignment() {
    /* Declare a `season` variable set to a string with your favorite season. Provide an explicit type annotation. The type of a string is a `&str`. We'll discuss what
    the & symbol means later in the course.*/
    let season: &str = "Spring";

    /* Declare a `points_scored` variable set to 28. Provide an explicit type annotation. The type of an integer is `i32`.*/ 
    let mut points_scored = 28;

    // It's time to update the team's score. Declare the `points_scored` variable to be mutable. Set its new value to 35.
    points_scored = 35;

    // Declare a `TOUCHDOWN_POINTS` constant at the file level set to the value 6.

    // Declare a `event_time` variable set to a string of "06:00".
    let event_time = "06:00";

    // Use variable shadowing to redeclare `event_time` set to a integer of 6.
    let event_time: i32 = 6;

    /* Use interpolation to print out all of the declared variables and constants in a println! call. Practice using direct interpolation {value}, sequential
    arguments ( {} ), and numeric arguments ( {0} ).*/
    println!("{season} season, touch-down points were {0} at the time the game was starting at {1}. 
    The points scored by the end of the match were {2}", TOUCHDOWN_POINTS, event_time, points_scored);

    // Declare a `favorite_beverage` variable set to a string of your favorite drink. Use an underscore to silence the compiler warning about the variable being unused.
    let favorite_beverage: &str = "Tursker Beer";

    // Remove the underscore. Provide a compiler directive to silence the compiler warning about the variable being unused.

}



fn main() {
    let apples = 30;
    let mangos = 13;
    let fruits = apples + mangos;

    println!("The apples for this season are {} in total", apples);
    println!("The mangos for this season are {mangos} in total");
    println!("There are {} apples, {} mangos and a total of {} fruits", apples, mangos, fruits);
    println!("There are {0} apples, {1} mangos and a total of {2} fruits", apples, mangos, fruits);


    // mutability in variables using the mut keyword. Variables are immutable by default in rust
    let mut gym_reps = 10;
    println!("I will attempt {} reps for chest press", gym_reps);

    gym_reps = 15;
    println!("Net week I will improve to {} reps of the same movement", gym_reps);

    //To learn more about errors use terminal rustc --explain <error code> or in the browser, search for rust error codes

    //Variable Shadowing - redeclaring a variable using the same name
    let _grams_of_protein = "100.345"; //the variable starts-out as a string
    let _grams_of_protein = 100.345; // Shadowing allows us to reassign the variable to a float value
    let _grams_of_protein = 100; //Further utilization of scope allows reassignment into an integer value

    //Variable Scope
    let coffee_price = 35.2; //this is accessible by the inner block / inner scope
     {
        let cookie_price = 12; // this is not accessible by the outer block / outer scope
        println!("The coffee price is {} and the cookie price is {}", coffee_price, cookie_price)
     }

    //Using the constant
    println!("The PAYE tax rate is {TAX_RATE}% in kenya");

    //Using Directives
    #[allow(unused_variables)]
    let unsused_variable = "Unused";

    assignment();

}
