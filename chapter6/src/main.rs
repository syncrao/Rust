fn main() {
    println!("chapter 6 Enum");
    let white = Message::White(String::from("White"));
    white.call();
    let mov = Message::Move { x: 6, y: 4 };
    if let Some(total) = mov.call() {
        println!("Total move value: {}", total);
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

impl Message {
    fn call(&self) -> Option<i32> {
        match self {
            Message::Quit => {
                println!("Quit");
                None
            },
            Message::Move { x, y } => {
                let total = x + y;
                Some(total)
            },
            Message::White(value) => {
                println!("white : {}", value);
                None
            },
            Message::ChangeColor(r, g, b) => {
                println!("Change color to rgb({}, {}, {})", r, g, b);
                None
            },
        }
    }
}


