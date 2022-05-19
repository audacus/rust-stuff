use iterators::Counter;

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    let c1 = Counter::new();
    let c2 = Counter::new().skip(2);
    c1.zip(c2).for_each(|(a, b)| println!("{} -> {}", a, b));
}