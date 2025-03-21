fn main() {
    let favorite_color: Option<String> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = Err("new");

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!")
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!{"{top}"}
    }
    
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let x = Some(5); 
    let y = 10; 

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _=> println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 5;

    match x {
        1 | 2 => println!("one or two"),
        1..=5 => println!("three"),
        _=> println!("Other"),
    }

    let x = Some(5);

    match x {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (), 
    }

    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello{id:101};

    match msg {
        Message::Hello {id: id_variable @ 3..=7,} => println!("Found an id in range: {id_variable}"),
        Message::Hello {id: 10..=12} => println!("Found an id in another range"),
        Message::Hello {id} => println!("Found some other id: {id}"),
    }


}
