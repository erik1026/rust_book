fn main() {
    
    // Creates a vec with no values in it. We have to specify the data type
    // if there are no values when initialized
    {
        let _v: Vec<i32> = Vec::new();
    }

    // Creating a vec with values using the 'vec!' macro
    {
        let _v = vec![1, 2, 3];
    }

    // Updating a vec
    {
        let mut v = Vec::new(); // The type doesn't need to be specified because we add
                                          // values later so the compiler can infer it's type

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    // Reading elements of vectors
    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }
    }

    // Iterating over the values in a Vector
    {
        // Immutable references
        let mut v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }

        // Mutable references
        for i in &mut v {
            *i += 50;
        }
    }

    // Using an enum to store multiple types
    {
        enum SpreadsheetCell{
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

    }

}
