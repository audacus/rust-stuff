fn main() {
    another_function(5, 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let (a, b) = (x, y);
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}
