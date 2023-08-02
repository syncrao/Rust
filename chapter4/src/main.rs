fn main() {
    let s1 = String::from("hello");
    let len = calculate_langth(&s1);
    println!("the length of {}, is {}.", s1, len);
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // println!("word is {}", word);
    // s.clear(); 

    let my_string = String::from("Hello world!!");
    let word1 = first_word(&my_string[..]);
    println!("word is {}", word1);
    let my_string_literal = "Hello world";
    let word1 = first_word(&my_string_literal[..]);
    println!("word is {}", word1);
    let word1 = first_word(my_string_literal);
    println!("word is {}", word1);
}

fn calculate_langth(s: &String) ->  usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
        
    }
    &s[..]
}