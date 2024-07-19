extern crate chapter7;
use chapter7::{add, info, user};



fn main() {
    println!("Chapter 7 module");
    add(10, 20);

    let blance = info::UserInfo::Blance{rs: 30, usd: 10};
    blance.call();

    let stocks = info::UserInfo::Stock{ind: "TATA".to_string(), usa: "TESLA".to_string()};
    stocks.call();

    let user1 = user::user_list::User {
        name: String::from("Shah Rukh RA0"),
        email: String::from("shaho@gmail.com"),
        age: 30,
        active: true,
    };

    let userinfo = info::UserInfo::User(user1);
    userinfo.call();

}