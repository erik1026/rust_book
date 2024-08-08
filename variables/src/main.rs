const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // This would fail of 'x' was not a mutable variable
    println!("The value of x is: {x}");

    println!("Using our constant: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
