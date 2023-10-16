fn main() {
    {
        // passing by reference
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}", s1, len);
    }
    {
        // passing by mutable reference
        let mut s = String::from("hello");
        change(&mut s);

        let r1 = &mut s;
        // let r2 = &mut s; -> cannot borrow second mutable
        let r2 = &mut &r1;

        println!("{} / {}", r1, r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        // let r2 = &mut s; -> cannot borrow as mutable if also using immutable
        let r2 = &s;

        println!("{}, {}, and {}", s, r1, r2);
        // immutable r1 & r2 are not used anymore

        let r3 = &mut s;
        println!("{}", r3); // allowed since immutable references are not anymore used
        // println!("{}", r1) -> doesn't work -> mutable / immutable at same time
    }

    {
        // trying to dangle
        // let reference_to_nothing = dangle();
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// trying to dangle
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // return reference to s
} // s goes out of scope -> invalid
*/