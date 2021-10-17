struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let user1 = User {
        username: String::from("test"),
        email: String::from("test@mail.com"),
        active: false,
        sign_in_count: 5
    };

    let _user2 = build_user(String::from("test2"), String::from("test2@mail.com"));

    let _user3 = User {
        username: String::from("test3"),
        email: String::from("test3@mail.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);


    let rectangle1 = Rectangle {
        width: 5,
        height: 10
    };

    let area = get_area(&rectangle1);

    println!("Rectangle area is: {}", area);
    println!("Rectangle: {:#?}", rectangle1);


    let rectangle2 = Rectangle {
        width: 3,
        height: 4
    };

    let area = rectangle2.area();

    println!("#2 Rectangle area is: {}", area);
    println!("#2 Rectangle: {:#?}", rectangle2);


    let can_2_fit_inside_1 = rectangle1.can_hold(&rectangle2);

    println!("Can 2 fit inside 1? {}", can_2_fit_inside_1);


    let square = Rectangle::square(15);

    println!("Here's a square: {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn get_area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

