#![allow(unused, dead_code)]

fn maybe<T>(x: T) -> Option<T> {
    if std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() % 2 == 0 {
        Some(x)
    } else {
        None
    }
}

fn do_it(value: Option<Foo>) {
    if let Some(value) = value {
        drop(value)
    }
}

#[derive(Debug, Default)]
struct Foo(u32);

fn main() {
    let flag = true;
    do_it(flag.then(|| Foo(42)))
}
