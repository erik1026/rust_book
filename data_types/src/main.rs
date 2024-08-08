fn main() {
    // This line would not work if we didn't annotate the type with 'u32'
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // Scalar Types
    // floating point
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // numeric operations
    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 /32.2;
    let _trucated = -5 /3; // Results in -1

    let _remainder = 43 % 5;

    // boolean type
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // character type
    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound types
    // tuple
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = _tup;

    println!("The value of y is: {_y}");

    let _five_hundred = _tup.0;

    let _six_point_four = _tup.1;

    let _one = _tup.2;

    let mut _x2: (i32, i32) = (1, 2);
    _x2.0 = 0;
    _x2.1 += 5;

    // Array type
    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    let _c = [3; 5];

    let _first = _a[0];
    let _second = _a[1];
}
