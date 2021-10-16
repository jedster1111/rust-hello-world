fn main() {
    println!("Hello, world!");

    let x = five();

    let y = {
        let x = x;
        plus_one(x)
    };

    another_function(x, y);
}

fn five() -> u32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}

fn another_function(x: u32, y: u32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}
