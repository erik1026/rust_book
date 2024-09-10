struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,       // Shorthand for when the value is the same name as the field
        email,
        sign_in_count: 1,
    }
}

// Tuple Structs without names fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    // ----- Defining and Instantiating Structs -----
    {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");

        // Creating another user based on user1
        // let user2 = User {
        //     active: user1.active,
        //     username: user1.username,
        //     email: String::from("another@example.com"),
        //     sign_in_count: user1.sign_in_count,
        // };

        // Create another user based on user1 but using the Update Syntax.
        // This moves the data to the new variable and the old variable is now
        // invalid
        let _user3 = User{
            email: String::from("another@example.com"),
            ..user1
        };
    }

    {
        let _black = Color(0, 0, 0);
        let _origin = Point(0, 0, 0);
    }

    {
        let _subject = AlwaysEqual;
    }

    // ----- Example Program using structs -----
    {
        let width1 = 30;
        let height1 = 50;

        let rect1 = Rectangle {
            width: width1,
            height: height1
        };

        println!("rect1 is {rect1:?}");

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );
    }

    // ----- Method Syntax -----
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels",
            rect1.area()
        );

        if rect1.width() {
            println!(
                "The rectangle has a nonzero width: it is {}",
                rect1.width
            );
        }

        let rect2 = Rectangle {
            width: 10,
            height: 40
        };

        let rect3 = Rectangle {
            width: 60,
            height: 45
        };

        println!(
            "Can rect1 hold rect2? {}",
            rect1.can_hold(&rect2)
        );

        println!(
            "Can rect1 hold rect3? {}",
            rect1.can_hold(&rect3)
        );

    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Using a tuple
// s

// Using a struct
fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}
