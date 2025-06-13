use crate::user::user_list::User;

pub enum UserInfo {
    Blance {rs: u32, usd: u32},
    Stock {ind: String, usa: String},
    User (User)
}

impl UserInfo {
   pub fn call(&self) {
    match self {
        UserInfo::Blance {rs, usd} => {
            println!("Rupay: {}, Doller: {}", rs, usd);
        },
        UserInfo::Stock {ind, usa} => {
            println!("Indian Stock Name: {}, USA Stock Name {}", ind, usa);
        },
        UserInfo::User(user) => {
            println!("Name: {}, Email: {}, Age: {}", user.name, user.email, user.age);
        }
    }
   }
}