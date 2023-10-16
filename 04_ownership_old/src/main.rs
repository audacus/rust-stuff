fn main() {
    let l = "hello"; // type "str" --> string literal --> immutable, not stored on heap
    let l = l.to_owned() + ", world!";
    println!("{}", l);

    let mut s = String::from("hello"); // type "String" --> string object --> stored on heap

    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s); // This will print `hello , world!`

    // stored on heap
    let s1 = String::from("hello");
    let s2 = s1; // copies only reference, invalidates s1 (moved) (shallow copy)

    // println!("{}, world!", s1); // error: borrow value after move
    println!("{}, world!", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone(); // creates new data on heap (deep copy)

    println!("s3 = {}, s4 = {}", s3, s4);

    // stored on stack
    let x = 5;
    let y = x; // creates copy

    println!("x = {}, y = {}", x, y);

    let e = String::from("example"); // e comes into scope
    takes_ownership(e); // e's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}", e); // fails due to moved value

    let ef = String::from("bla");
    no_taking_ownership(&ef); // pass by reference
    println!("{}", ef);

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    let _s5 = gives_ownership(); // gives_ownership moves its return value into s1

    let _s6 = String::from("hello"); // s2 comes into scope

    let _s7 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    // not using references
    let something1 = String::from("bla bla bla bla");
    let (something2, len) = calculate_length(something1);
    println!("The length of '{}' is {}.", something2, len);

    let len = calculate_length_ref(&something2);

    println!("The length of '{}' is {}.", something2, len);

    let mut hello = String::from("hello");
    change(&mut hello);

    {
        let r1 = &mut hello;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &hello;
    println!("{}", r2);
    // r2 is no longer used after this point

    let r3 = &mut hello;
    println!("{}", r3);

    let _from_fn = no_dangle();

    let mut words = String::from("hello world");

    let _word = first_word(&words); // word will get the value 5

    // words.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    //
    let _hello = &words[0..5];
    let _world = &words[6..11];

    let _slice_begin = &words[0..2];
    let _slice_begin = &words[..2];

    let len = words.len();
    let _slice_all = &words[0..len];
    let _slice_all = &words[..];

    let first = first_word(&words);

    // words.clear(); // error!

    println!("the first words is: {}", first);

    let _my_string = String::from("hello world");

    // first_word works slices of `String`s

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _word = first_word(&my_string_literal[..]);

    // Becaue string literals *are* string slices already, this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn no_taking_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello");
    some_string // some_string is returned
}

fn takes_and_gives_back(a_string: String) -> String {
    // takes_and_gives_back will take a String and return one
    a_string // a_string is returned
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word_old(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // byte literal syntax
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
