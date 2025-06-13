use std::thread;
use std::time::Duration;


fn main() {

    let simulated_intensity = 10;
    let simulated_random_number = 17;

    generate_workout(simulated_intensity, simulated_random_number);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    
}

struct Cacher<T> where T: Fn(u32) -> u32, {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
where 
    T: Fn(u32) -> u32, 
{

    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation: calculation, value: None}
    }

    fn value(&mut self, arg:u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }

}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly .. ..");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pishups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today");
        } else {
            println!("Today, run fro {} minutes!", cached_result.value(intensity) );
        }
    }
}


trait Iterator  {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn idemon() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn summ() {
    let v1 = vec![1,2,33];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 36);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filter_size() {
    let shoes = vec![
        Shoe{
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe{
            size: 13,
            style: String::from("sandal"),
        },
        Shoe{
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_size = shoes_size(shoes, 10);

    assert_eq!(
        in_size, vec![
            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 10,
                style: String::from("boot"),
            },
        ]
    )


}