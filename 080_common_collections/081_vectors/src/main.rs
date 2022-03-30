fn main() {
    {
        let v: Vec<i32> = Vec::new();
    }

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        // let does_not_exist = &v[100]; -> panic
        if let Some(number) = v.get(2) {
            println!("Value: {}", number);
        } else {
            println!("There is none!");
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        // v.push(6); -> cannot borrow mutable (might has to rearange vector on heap -> reference to first value could get invalid)

        println!("The first element is: {}", first);
    }
}
