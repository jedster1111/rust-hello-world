fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);


    let i1 = 5;
    let i2 = i1;

    println!("i1: {}, i2: {}", i1, i2);


    let s1 = String::from("hello");

    takes_ownership(s1);

    let _s2 = s1; // <- Fails due to S1 being moved into `takes_ownership`


    let i1 = 5;

    makes_copy(i1);

    let _i2 = i1; // <- Doesn't fail since i32 implements copy


    let s1 = String::from("hello");

    let s1 = takes_ownership_and_returns_ownership(s1);

    let _s2 = s1; // <- Since we returned ownership, no errors here


    let s1 = String::from("hello");

    reference_example(&s1);

    let _s2 = s1; // <- Since we passed in by reference, no problem accessing s1 at this point


    let mut s = String::from("Hello world!");

    let word = first_word(&s);
    
    s.clear(); // <- Fails due to a mix of mutable & immutable borrowing
    
    println!("the first word is: {}", word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership_and_returns_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn reference_example(some_string: &String) {
    println!("{}", some_string);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    };

    &s[..]
}
