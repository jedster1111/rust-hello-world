// https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/rust

fn main() {
    println!("Result: {}", repeat_str("Hello world!", 5));
}

fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}
