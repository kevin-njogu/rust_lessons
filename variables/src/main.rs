fn main() {
    let apples = 30;
    let mangos = 13;
    let fruits = apples + mangos;

    println!("The apples for this season are {} in total", apples);
    println!("The mangos for this season are {mangos} in total");
    println!("There are {} apples, {} mangos and a total of {} fruits", apples, mangos, fruits);
    println!("There are {0} apples, {1} mangos and a total of {2} fruits", apples, mangos, fruits);
}
