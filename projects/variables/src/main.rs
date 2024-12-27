fn main() {
    // variables are immutable by default
    // use mut to make them mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants are always immutable
    // they are declared with the const keyword
    // they use screaming snake case by convention
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    // you can declare a new variable with the same name as a previous variable
    // the new variable shadows the previous variable
    // the new variable can have a different type than the previous variable
    let spaces = "   ";
    let spaces = spaces.len();
    print!("The number of spaces is: {spaces}");
}
