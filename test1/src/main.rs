
fn main(){

    let x: u32 = 2;
    let y: u32 = 2;

    match x {
       y => println!("yes")
       `y => println!("No")
    };
    let z  = x.cmp(&y);
    match x {
        y => println!("yes")
        `z => println!("No")
     };
    println!("{:?}", z)
}