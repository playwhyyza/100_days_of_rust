fn main() {
    // Mutable and Immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // global shadowing scope
    let shadow_x = 5;

    let shadow_x = shadow_x + 1;

    {
        // inner shadowing scope
        let shadow_x = shadow_x * 2;
        println!("The value of x in the inner scope is: {shadow_x}");
    }

    println!("The value of x is: {shadow_x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces: {spaces}")
}
