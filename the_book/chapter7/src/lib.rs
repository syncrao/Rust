
mod home;
pub mod user;
pub mod info;
pub fn add(left: usize, right: usize) {
    println!("done");
    let home = left + right;
    home::home(home);
    user::user();

    let user1 = user::user_list::User {
        name: String::from("Shah Rukh"),
        email: String::from("rao@gmail.com"),
        age: 29,
        active: true,
    };

    user1.user_details();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
