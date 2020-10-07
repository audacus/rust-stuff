fn main() {
    let l = "hello"; // string literal --> immutable, not stored on heap
    let l = l.to_owned() + ", world!";
    println!("{}", l);

    let mut s = String::from("hello"); // string object --> stored on heap

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
