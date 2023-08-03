fn main() {
    let mut user1 = User {
        email: String::from("shahrukhrao1@gmail.com"),
        username: String::from("shah_rukh_rao"),
        active: true,
        sign_in_count: 1,
    };
    println!("username {} email {}", user1.username, user1.email);
    user1.username = String::from("shah_rukh");
    println!("username {} email {}", user1.username, user1.email);
    println!("active {}, count {}", user1.active, user1.sign_in_count);

    let mut user2 = User {
        username: String::from("shah_rukh_rao"),
        ..user1
    };
    println!("username {} email {}", user2.username, user2.email);
    println!("active {}, count {}", user2.active, user2.sign_in_count);

    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);
    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    // println!("The area of the rectangle is {} square pixels.", area(rect1));
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The area of the rectangle is {:#?} square pixels.", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(dimentions: (u32, u32)) -> u32 {
//     dimentions.0 * dimentions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
