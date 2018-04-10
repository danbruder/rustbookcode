#![allow(unused_variables)]

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
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
            _ => 34,
        }
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    dothings(coin, &mut count);
    let coin = Coin::Penny;
    dothings(coin, &mut count);
    let coin = Coin::Nickel;
    dothings(coin, &mut count);
    println!("{}", count)
}

fn dothings(coin: Coin, count: &mut u32) {
    if let Coin::Quarter(state) = coin {
        println!("{:?}", state);
    } else {
        *count = *count + 1;
    }
}
