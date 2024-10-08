// non-generic largest function
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic function
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Structs
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl that uses multiple types
// impl <X1, Y1> Point<X1, Y1>{
// fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point { x: self.x, y: other.y }
//     }
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Defining functions on specific types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // Using non-generic functions
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {result}");
    }

    // Using the generic function
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}");
    }

    // Structs
    {
        let _integer = Point { x: 5, y: 10 };
        let _float = Point { x: 1.0, y: 4.0 };
    }

    // Structs with multiple types
    // {
    //     let _both_interger = Point { x: 5, y: 10 };
    //     let _both_float = Point { x: 1.0, y: 4.0 };
    //     let _integer_and_float = Point { x: 5, y: 4.0 };
    // }

    {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
}
