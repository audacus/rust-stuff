fn main() {
    {
        let s1 = "string literal"; // string literals -> stack
        let mut s2 = String::from("hello"); // String (Drop) -> heap
        s2.push_str(", world!");
        println!("{}", s1);
        println!("{}", s2);

        let s1 = "hello";
        println!("{}", s1);
    } // here s and ss go out of scope -> no longer valid

    {
        let x = 5; // primitive data types (Copy) -> stack
        let y = x; // copies value
        println!("x = {}, y = {}", x, y);

        let s1 = String::from("hello"); // String (Drop) -> heap
        // let s2 = s1; error on println -> value borrowed after move
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);

        takes_ownership(s1); // s1 is movved into the function -> no longer valid
        makes_copy(x); // i32 is Copy -> gets copied into function -> still valid
    } // here x, y, s1, s2 go out of scope -> no longer valid

    {
        let _s1 = gives_ownership();
        let s2 = String::from("hello");
        let _s3 = takes_and_gives_back(s2);
    } // s1 & s3 goes out of scope, s2 was moved

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1); // get tuple
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope -> `drop` is called -> no longer valid

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope -> nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
} // move value into function that calls it

fn takes_and_gives_back(a_string: String) -> String { // moves in
    a_string
} // moves out

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // return tuple
}