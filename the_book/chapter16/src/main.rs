use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;

use std::sync::{Mutex, Arc};


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 2;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {}", *counter.lock().unwrap())

//    let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move ||{
//         let vals = vec![String::from("hello"), String::from("from"), String::from("the"), String::from("threads") ];
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });


//    thread::spawn(move ||{
//     let vals = vec![String::from("hello2"), String::from("from2"), String::from("the2"), String::from("threads2") ];

//     for val in vals {
//         tx.send(val).unwrap();
//         thread::sleep(Duration::from_secs(2));
//     }
//    });

//    for recieved in rx {
//         println!("Got {recieved}");
//    }

}
