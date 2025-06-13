extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("This is my First Program in Rust after Complete chapter 3 ");
    name_age();
}

fn name_age() {
    let mut name = String::new();
    let mut age = String::new();

    println!("Enter your name ");
    std::io::stdin().read_line(&mut name).expect("Failed");
    println!("Enter your age ");
    std::io::stdin().read_line(&mut age).expect("Failed");
    println!("Welcome {}", name);

    loop {
        println!("if you want to calculate your plot size type (plot) OR if you want to play game type (game)");
        let mut typ = String::new();
        std::io::stdin().read_line(&mut typ).expect("Failed");
        let typ = typ.trim();
        if typ == "plot" {
            plot_size();
        } else if typ == "game" {
            game();
        } else {
            println!("Your name is {} Your age is {}", name, age);
            break;
        }
    }

    let age: u32 = age.trim().parse().expect("failed");
    let name_len = name.trim().len();

    let age = 2023 - age;
    println!("Your birth year is {}", age);

    if name_len % 2 == 0 {
        println!("Your name length is {} and Number is Even ", name_len);
    } else {
        println!("Your name length is {} and Number is odd ", name_len);
    }
}

fn plot_size() {
    println!("Enter your plot length ");
    let plot_len = convert();
    println!("Plot Length {}", plot_len);

    println!("Enter your plot Width ");
    let plot_wid = convert();
    println!("Plot Length {}", plot_wid);

    let size = (plot_len * plot_wid) / 4;
    println!("Your plot size is in gaj {}", size)
}

fn convert() -> u32 {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed");
    let name: u32 = name.trim().parse().expect("failed");
    name
}

fn game() {
    println!("Guess the number ");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess ");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guess {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
