#![allow(unused)]

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main() {
    let val = Arc::new(vec![42, 10]);
    let mut change = Arc::new(Mutex::new(52));
    let t = vec![
        std::thread::spawn({
            let val = Arc::clone(&val);
            let change = change.clone();
            move || {
                println!("work: {}", val[0]);
                *change.lock().unwrap() += 10;
            }
        }),
        std::thread::spawn({
            let val = val.clone();
            move || {
                println!("work: {}", val[1]);
            }
        }),
    ];
    dbg!(&val);

    println!("other work");
    for thread in t {
        println!("{:?}", thread.join().unwrap());
    }
    dbg!(change);
}
