fn main() {
    
    {                               // s is not valid here, it's not yet declared
        let _s = "hello";     // s is valid from this point forward
    
        // do stuff with s
    }                               // this scope is now over, and s is no longer valid

    {
        let mut _s = String::from("hello"); // s is valid from this point forward

        _s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{_s}"); // This will print 'hello, world!'
    } // this scope is now over, and s is no longer valid
    

    // variables and data interacting with Move
    {
        let x = 5;
        let _y = x;

        // This assigns the value of '5' to 'x'. The value of 'x' is then copied and assigned to 'y'
        // both variables live on the stack

        let s1 = String::from("hello");
        let s2 = s1;

        // Here, space is allocated on the heap to store the String value. On the stack
        // 3 things are being stored related to 's1'. The pointer to the heap, the length of the data,
        // and the capacity of the data. When we assign 's1' to 's2', the stack data is being copied,
        // not the data from the heap. Because only one thing is allowed to own something, 's1' is no longer
        // valid and will cause an error at compile time

        // println!("{s1}"); // will cause an error 
        println!("{s2}"); // valid code
    
        // This would be akin to a 'shallow copy' in other languages but invalidates the previous variable
        // which is why it is called a 'move' instead
    }

    // Variables and Data interacting with Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // This performs a deep copy 

        // Due to performing a 'deep copy', 's1' is still valid
        println!("s1 = {s1}, s2 = {s2}");
    }

    // Stack-Only Data: Copy
    {
        let x = 5;
        let y = x;

        println!("x = {x}, y = {y}");

        // This code is valid because 'x' lives on the stack and is an integer, which the compiler
        // knows the size of and that size doesn't change. Because of this, it is not expensive to copy
        // the data and thus doesn't need to explicitly call 'clone' to copy the data 
    }

    // Ownership and Functions
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s);     // s's value moves into the function...
                                            // ... and so is no longer valid here 

        let x = 5;  // x comes into scope

        makes_copy(x); // x would move into the function, but i32 is Copy,
                                    // so it's okey to still use x afterward
    } // Here, x goes out of sope, then s. but because s's value moved, nothing special happens

    // Return Values and Scope
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which
                                                             // also moves its return value into s2
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped

    // ----- References and Borrowing -----
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}");
    }

    // Mutable references
    {
        let mut s = String::from("hello");

        change(&mut s);

        println!("{s}");

        // let r1 = &mut s;
        // let r2 = &mut s;

        // println!("{r1}, {r2}");
    } // You can't have multiple mutable references floating around 

    {
         let mut s = String::from("hello");

    //     let r1 = &s; // no problem
    //     let r2 = &s; // no problem
    //     let r3 = &mut s; // BIG PROBLEM
    
    //     println!("{}, {}, and {}", r1, r2, r3);
    
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1}, and {r2}");
        
        let r3 = &mut s; // no problem because r2 and r1 are no longer being used in this scope
        println!("{r3}");
    }
    // We can't have a mutable reference at the same time we have an immutable one

    // ----- The Slice Type -----
    // String slices
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        // There are the same
        let slice_one = &s[0..2];
        let slice_two = &s[..2];

        let len = s.len();
        let slice_three = &s[3..len];
        let slice_four = &s[3..];

    }

    {
        let my_string = String::from("hello world");

        let _word = first_word(&my_string[0..6]);
        let _word = first_word(&my_string[..]);

        let _word = first_word(&my_string);

        let my_string_literal = "hello world";

        let _word = first_word(&my_string_literal[0..6]);
        let _word = first_word(&my_string_literal[..]);

        let _word = first_word(my_string_literal);
    }
}


// ----- Functions for 4.1 -----
fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

// ----- Functions for 4.2 -----
fn calculate_length(s: &String) -> usize { // s is a referencce to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

// fn dangle() -> &String { // returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped. Its memory goes away. DANGER

fn _no_danlge() -> String {
    let s = String::from("hello");

    s
}

// ----- Functions for 4.3 -----
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}