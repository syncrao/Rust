use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess  number Game !");
    let secret_number = rand::thread_rng().gen_range(1..=100);
   
    loop {
    println!("please input your Guess.");
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line ");

    let guess: u32 = match guess.trim().parse() {Ok(num)=> num, Err(_) => continue};

    println!("You guessed:{}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small Number"),
        Ordering::Greater => println!("Too Big Number"),
        Ordering::Equal => {println!("You Win"); break}
    }
}
}

