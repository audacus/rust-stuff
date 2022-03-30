use rand::Rng;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    {
        println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let dice_roll = rand::thread_rng().gen_range(1, 11);
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            4..=8 => move_player(dice_roll),
            1 | 2 => reroll(),
            _ => ()
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("add fancy hat")
}
fn remove_fancy_hat() {
    println!("remove fancy hat")
}
fn move_player(num_spaces: u8) {
    println!("move player {} spaces", num_spaces)
}

fn reroll() {
    println!("reroll")
}
