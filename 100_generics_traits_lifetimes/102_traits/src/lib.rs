use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}


impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

// TODO: ???
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         format!("{}", &self)
//     }
// }


pub mod aggregator {

    use std::fmt::{Debug, Display};

    pub trait Summary {
        fn summarize(&self) -> String;

        fn read_more_author(&self) -> String;

        fn read_more(&self) -> String {
            format!("(Read more from {}...)", self.read_more_author())
        }
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

        fn read_more_author(&self) -> std::string::String {
            format!("{}", self.author)
        }
    }

    pub struct Tweet  {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn read_more_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    pub fn notify(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {

    }

    pub fn notify_verbose<T: Summary + Display>(item: &T) {
        println!("What?! {}", item.summarize());
    }

    pub fn notify_two_verbose<T: Summary>(item1: &T, item2: &T) {

    }

    fn some_function<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {
        4
    }

    fn return_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course people"),
            reply: false,
            retweet: false,
        }
    }

    fn return_summarizable_switch(switch: bool) -> impl Summary {
        let news = NewsArticle {
            headline: String::from("Penguins"),
            location: String::from("Pittsburgh"),
            author: String::from("Iceburgh"),
            content: String::from("gg easy"),
        };

        if switch {
            news
        } else {
            // if else incompatible types (NewsArticle / Tweet)
            /*
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course people"),
                reply: false,
                retweet: false,
            }
            */
            NewsArticle {
                location: String::from("Unknown"),
                content: String::from("gl hf"),
                ..news
            }
        }
    }
}