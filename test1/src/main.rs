fn main() {
    let s = String::from("hello");
    let mut s1 = s;
    print_world(&mut s1);
    println!("{s1}")
}

fn print_world( word:  &mut String) {
    word.push_str(" World");
    println!("{word}");
}