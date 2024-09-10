enum IpAddr {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // ----- Defining an Enum -----
    {
        let _four = IpAddr::V4;
        let _six = IpAddr::V6;

        // let home = IpAddr::V4(String::from("127.0.0.1"));
        let _home = IpAddr::V4(127, 0, 0, 1);

        let _loopback = IpAddr::V6(String::from("::1"));
    }

    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // Option enum
    {
        // The option enum takes on the type of whatever data the Some variant holds
        let _some_number = Some(5);
        let _some_char = Some('e');

        // If the Option is a 'None' variant then it needs to be explicitly stated
        // what type of value could be stored
        let _absent_number: Option<i32> = None;
    }

    // ----- Match control flow Construct -----
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        println!("{six:?}");
        println!("{none:?}");
    }

    // ----- Concise Control Flow with if let -----
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
