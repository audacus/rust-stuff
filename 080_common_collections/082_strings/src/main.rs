#![allow(unused)]
fn main() {
    {
        let mut s = String::new();
    }

    {
        let s = "initial contents".to_string();
        let s = String::from("initial contents");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is '{}', s1 is '{}'", s2, s1);

        s1.push('s');
        println!("{}", s1);
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world");
        let str1 = "!";
        let s3 = s1 + &s2 + str1; // s1 has been moved -> invalidated
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);

        println!("{}", s);
    }

    {
        let hello_es = "Hola!";
        let s_es = &hello_es[0..4];
        println!("first 4 bytes of '{}': {}", hello_es, s_es);

        let hello_ru = String::from("Здравствуйте");
        let s_ru = &hello_ru[0..4];

        println!("first 4 bytes of '{}': {}", hello_ru, s_ru);

        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
