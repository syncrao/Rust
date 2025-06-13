fn main() {
    println!("chapter 6 Enum");
    let white = Message::White(String::from("White"));
    match white.call() {
        MessageReturn::Name(name) => println!("name : {}", name),
        _ => (),
    }
    let mov = Message::Move { x: 6, y: 4 };
    match mov.call() {
        MessageReturn::Total(total) => println!("Total move value: {}", total),
        _ => (),
    }
    let rbg = Message::ChangeColor(123, 133, 333);
    rbg.call();
    let quit = Message::Quit;
    quit.call();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    White(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum MessageReturn {
    None,
    Total(i32),
    Name(String),
}

impl Message {
    fn call(&self) -> MessageReturn {
        match self {
            Message::Quit => {
                println!("Quit");
                MessageReturn::None
            },
            Message::Move { x, y } => {
                let total = x + y;
                MessageReturn::Total(total)
            },
            Message::White(value) => {
                println!("white : {}", value);
                let name = "shah rukh".to_string();
                MessageReturn::Name(name)
            },
            Message::ChangeColor(r, g, b) => {
                println!("Change color to rgb({}, {}, {})", r, g, b);
                MessageReturn::None
            },
        }
    }
}


