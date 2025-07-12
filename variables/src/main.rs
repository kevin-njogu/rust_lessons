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



}
