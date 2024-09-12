use std::fmt::{Debug, Display};

// Traits can call functions in the same trait even if they don't
// have a default implementation
pub trait Summary {
    fn summarize(&self) -> String;
    // Default implementation
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)")
    // }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Summary")
    }
}

// functions that take traits as input
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// multiple arguments that use the same trait and same type
// fn notify<T: Summary>(item1: &T, item2: &T){}
//
// multiple arguments that use the same trait but different types
// fn notify(item1: &impl Summary, item2: &impl Summary){}
//
// Multiple traits
// fn notify (item: &(impl Summary + Display)) {}
//
// fn notify<T: Summary + Display>(item: &T) {}

// Trait Bounds with 'where' clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {}
