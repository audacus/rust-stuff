use std::fmt::Display;

struct ImportantExerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    {
        let x = 5;
        let r = &x;

        println!("r: {}", r);
    }

    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
        // println!("The longest string is {}", result); -> error: string2 does not live long enough

    }

    {
        // &i32; -> reference
        // &'a i32 -> reference with explicit lifetime
        // &'a mut i32 -> mutable reference with explicit lifetime
    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExerpt {
            part: first_sentence,
        };
        i.announce_and_return_part("hello there");
    }

    {
        let s: &'static str = "I have a static lifetime.";
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest_with_an_announcement(
            string1.as_str(),
            string2,
            "Today is someone's birthday!",
        );
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}