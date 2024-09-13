struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // All lifetimes are accounted for by rules 1 and 3
    fn announce_and_return_part(&self, annoucement: &str) -> &str {
        println!("Attention please: {annoucement}");
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Dangling references
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r: {r}");
    // }

    // Preventing the dangling reference
    {
        let x = 5;

        let r = &x;

        println!("r: {r}");
    }

    // Generic lifetimes in Functions
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(&string1, &string2);
        println!("The longest string is {result}");
    }

    {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {result}");
        }
    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let _i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // a 'static lifetime says that the reference could live for the
    // entirity of the program
    {
        let _s: &'static str = "I have a static lifetime";
    }
}
