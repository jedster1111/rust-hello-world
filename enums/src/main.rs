enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("Call!")
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn main() {
    let _four = IpAddrKind::V4(127, 0, 0, 1);
    let _six = IpAddrKind::V6(String::from("::1"));

    let quit = Message::Quit;

    quit.call();

    let coin = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(&coin);

    println!("Value of {:?} is: {}", coin, value);


    let two = Some(2);
    let three = plus_one(&two);
    let none = plus_one(&None);

    if let Some(3) = three {
        println!("Found 3!")
    } else {
        println!("Wasn't 3")
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1)
    }
}
