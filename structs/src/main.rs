struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32

}

fn Area (rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    println!("Hello, Structs!");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("Email address from struct is {}", user1.email);
    let user2 = build_user(String::from("entchana@gmail.com"), String::from("chanakaudaya"));
    println!("Username from struct 2 is {}", user2.username);

    println!("Strating the Area calculation block");
    let rect = Rectangle { width: 25, height :10};
    println!("The rectangle is {:#?}", rect);
    println!("The area of the rectangle is {}", rect.area());

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}