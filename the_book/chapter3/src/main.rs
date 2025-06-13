fn main() {
    let tup  = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x {} y {} z {} tup is {}", x, y, z, tup.1);
    let months = ["January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"];
    let first = 1;
    println!("{}", months[first]);
    let x = ree();
    let y = addnum(5, 6);
    another(x,y);
    let fnc = if x < y {x} else {y};
    println!("function {}", fnc);
    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        println!("month {}", months[number]);
        number = number - 1;
    }
    for month in months.iter().rev() {
        println!("montha {}", month);
    }
}

fn another(x: i32, y: i32) {
    println!("x is {} y is {}", x, y);
}

fn ree() -> i32{
    5
}

fn addnum(x:i32,y:i32) -> i32 {
    x + y
}
