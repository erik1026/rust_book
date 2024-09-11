use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        let team_name = String::from("Blue");
        let _score = scores.get(&team_name).copied().unwrap_or(0);

        // Iterating over a hashmap
        for (key, value) in &scores {
            println!("{key}: {value}")
        }
    }

    // Hashmaps and ownership
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get!
    }

    // Updating a hash map
    {
        // overwriting a value
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{scores:?}");

        // Adding a key and value only if a key isn't present
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:?}");

        // Updating a value based on the old value
    }
}
