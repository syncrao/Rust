fn main() {
    let number = vec![10, 20, 40, 30, 5];

    let largest_number = get_number(number);

    println!("largest number {}", largest_number );

    let number = vec!['y', 'q', 'z', 'r','t'];

    let largest_number = get_number(number);

    println!("largest number {}", largest_number );

    let p = Point{x: 10, y: 10.10};
    let p1 = Point{x:11, y: 111.11};
    let p3 = p.mixup(p1);

    println!("p.x {} , P.y {}", p3.x, p3.y  );


}


fn get_number<T: std::cmp::PartialOrd + Copy >(number: Vec<T>) -> T {
    let mut largest_number = number[0];
    for num in number {
        if num > largest_number {
            largest_number = num
        }
    }
    largest_number
}

struct Point<T, U> {
    x: T, 
    y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: self.x,
            y: other.y
        }
    }
}