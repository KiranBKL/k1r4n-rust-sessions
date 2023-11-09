//Traits are like Interfaces in other programming languages

use std::fmt::format;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())//if struct does not implement this method then default will be executed but aummarize_author() not implemented so that should be implement in struct impl
    }
}

//lets implement this trait for structs

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

    fn summarize_author(&self) -> String {
        format!("{}",self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//trait as parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// this canbe implementd like this:(trait bounds)
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

//we can specify multiple bounds using +
// pub fn notify<T: Summary + std::fmt::Display>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

//we can use where syntax to make it more clearer
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { this not readable so
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: std::fmt::Display,
          U: std::fmt::Debug,
    {
        9
    }

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}// but we cant do this for multiple structs lets discuss about this upcoming lecs
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }



fn main () {
    let article = NewsArticle {
        headline: String::from("Rust is great"),
        location: String::from("Kepler452"),
        author: String::from("Rust"),
        content: String::from("Rust is great"),
    };

    let tweet = Tweet {
        username: String::from("Rust"),
        content: String::from("Rust is great"),
        reply: false,
        retweet: false,
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());

    notify(&article);
    notify(&tweet);
}












