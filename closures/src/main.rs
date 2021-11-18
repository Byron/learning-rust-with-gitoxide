#![allow(dead_code)]

use std::ops::AddAssign;

fn main() {
    println!("{:?}", map("a", |v| v));
    println!("{:?}", map(4, |v| v + 1));
    fn add_one(v: u32) -> u32 {
        v + 1
    }
    println!("{:?}", map(4, add_one));
    println!(
        "{:?}",
        map("a".to_string(), |mut v| {
            v.push_str("hello");
            v
        })
    );

    let mut count = 0;
    println!(
        "{:?}",
        map("a".to_string(), move |mut v| {
            count += 1;
            v.push_str("hello");
            v
        })
    );
    dbg!(count);

    struct Closure<'a> {
        count: &'a mut u32,
    }
    impl<'a> Closure<'a> {
        fn call(&mut self) -> u32 {
            self.count.add_assign(1);
            *self.count
        }
    }
}
struct Iter<F> {
    map: F,
}

impl<F> Iter<F>
where
    F: FnMut(u32) -> u32,
{
    fn next(&mut self) -> u32 {
        (self.map)(42)
    }
}
fn map<T>(item: T, mut f: impl FnMut(T) -> T) -> T {
    let t = f(item);
    f(t)
}
