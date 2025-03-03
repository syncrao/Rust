fn main() {
    println!("Welcome to Rust Bank ");
    let mut user1 = Account{name:"Shah".to_string(), number: 1, balance: 100};
    user1.balance += create(100, 100);

    if user1.balance == 300 {
        println!("YEs");
    };

    println!("username {}, number {}, balance {}", user1.name, user1.number, user1.balance);

}

struct Account {
    name: String,
    number: u8,
    balance: u32
}

fn create(d: u32, balance: u32)  -> u32 {
    d + balance
}