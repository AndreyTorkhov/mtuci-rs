// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArtivale {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArtivale {

//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     };

    // let article = NewsArtivale {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };

    // println!("New article available! {}", article.summarize());
//     println!("1 new tweet: {}", tweet.summarize());
// }

// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {})", self.summarize_author())
//     }

// }

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }


// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }

// use std::fmt::Display;

// struct Pait<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {x, y}
//     }
// }

// impl<T: Display + PartialOrd> Pait<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }