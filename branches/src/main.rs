fn main() {
    let condition = true;

    let number = if condition {5} else {7};

    println!("Number was {}", number);

    if condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    loop_example();
    while_example();
    for_example();
}

fn loop_example() {
    println!("Loop example");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Loop result is {}", result);
}

fn while_example() {
    println!("While example");

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Out of the loop!");
}

fn for_example() {
    println!("For example");

    let a = [10, 20, 30, 40 ,50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Lift off!");
}
