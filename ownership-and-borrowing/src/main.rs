#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
struct Foo(String, String);

#[derive(Clone)]
struct Bar {
    hello: String,
    world: String,
}

fn kung_foo(_fighter: Foo) {}

fn bar_foo(Bar { hello, world }: Bar) {
    ()
}

fn main() {
    let x = Foo("hello".to_string(), "world".to_string());
    kung_foo(x.clone());

    let something_descriptive = {
        let mut y = 42;
        y += 1;
        y
    };

    dbg!({ Foo("hello".to_string(), "scope".to_string()) });
}
