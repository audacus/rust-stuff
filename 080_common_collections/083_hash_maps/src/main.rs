use std::collections::HashMap;

fn main() {
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name & field_value are invalid here
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

        scores.insert(String::from("Red"), 40);
        scores.insert(String::from("Green"), 60);

        println!("{:?}", scores);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        match score {
            Some(s) => println!("Score of team '{}': {}", team_name, s),
            None => println!("There is no score for Team '{}'", team_name),
        }

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    // overwrite value
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    // insert if has no value
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    // update value based on old value
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
