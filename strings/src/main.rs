fn main() {
    // Creating a new string
    {
        let mut _s = String::new();
    }

    // Creating a string from some initial data
    {
        let data = "initial contents";

        let _s = data.to_string();

        // The method also works on a literal directly
        let _s = "initial contents".to_string();

        let _s = String::from("initial contents");
    }

    // Updating a String
    {
        let mut s = String::from("foo");
        s.push_str("bar");

        // push_str doesn't take ownership
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");

        // push adds a single character to a String
        let mut s = String::from("lo");
        s.push('l');
    }

    // Concatenation with the '+' operator or the 'format!' Macro
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let _s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used

        // Adding multiple Strings with the 'add' operator gets unwieldy so we can use the 'format!' macro
        // 'format!' does not take ownership
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");

        println!("{s}");
    }

    // Iterating over strings
    {
        for c in "Зд".chars() {
            println!("{c}");
        }

        for b in "Зд".bytes() {
            println!("{b}");
        }
    }
}
