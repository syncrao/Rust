use std::collections::HashMap;

fn main() {
 
    let mut v = vec![1,2,3];
    v.push(5);
    let addval = [6, 7 , 8 , 9];
    for i in addval {
        v.push(i);
    }
    let getval: Option<i32> =  v.get(4).copied();
    println!("index value : {} get value : {:?}", v[3],getval);

    for i in &v {
        println!("each value: {}", i);
    }

    enum SheetCell {
        Int (i32), 
        Float (f64),
        Text (String)
    }

    let row = vec![
        SheetCell::Int(10),
        SheetCell::Float(20.10),
        SheetCell::Text(String::from("Sheeet"))
    ];

    for i in &row {
        match i {
            SheetCell::Int(val) => println!("Int Value: {}", val),
            SheetCell::Float(val) => println!("Float Value : {}", val),
            SheetCell::Text(val) => println!("Text Value : {}", val)
        }
    }

    //  hashmap

    let red = String::from("Red");
    let blue = String::from("Blue");

    let mut score = HashMap::new();

    score.insert(red.clone(), 10);
    score.insert(blue.clone(), 20);

    let tblue = String::from("Blue");

    match score.get(&tblue) {
        Some(val) => println!("Blue Score: {}", val),
        None => println!("Blue Score not found"),
    }

    let tred = String::from("Red");

    if let Some(&redval) = score.get(&tred) {
        println!("Team Name: Red, Score: {}", redval);
    } else {
        println!("Red Score not found");
    }

    let teams = vec![red, blue];

    for team in teams {
        match score.get(&team) {
            Some(&val) => println!("Team Name: {}, Score: {}", team, val),
            None => println!("Score for {} not found", team),
        }
    }

    let Text = "I m learnig Rust because Rust is Great lang";

    let mut map = HashMap::new();

    for i in Text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 2;
    }

    println!("{:?}", map);

}
