fn main() {
    // string slices
    {
        let mut string = String::from("hello world");

        let hello = &string[..5]; // slice from start to 5
        let world = &string[6..]; // slice from 6 to end
        println!("{}, {}!", hello, world);

        let string_word = first_word(&string);

        println!("first word is {}", string_word);
        string.clear();

        // string literals are string slice references -> &str
        let string_literal = "hello world";

        let literal_word_slice = first_word(&string_literal[6..]);
        let literal_word = first_word(string_literal);

        println!("words: {}, {}", literal_word_slice, literal_word);
    }
    // other slices
    {
        let a = [1, 2, 3, 4, 5];
        println!("{:?}", a); // [1, 2, 3, 4, 5]
        let slice = &a[1..3];
        println!("{:?}", slice); // [2, 3]
        assert_eq!(slice, &[2, 3]);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return slice from start to index
        }
    }
    &s[..] // return slice from start to end
}