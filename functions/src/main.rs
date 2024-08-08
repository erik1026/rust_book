fn main() {
    println!("Hello, world!");

    another_function(5);

    // Statement and expressions
    //let x = (let y = 5); // This doesn't work because 'let' does not return a value

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // functions with return values
    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
