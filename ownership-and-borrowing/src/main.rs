#![allow(dead_code, unused_variables)]

#[derive(Clone)]
struct Foo {
    hello: String,
    world: String,
}

#[derive(Clone)]
struct Bar<'a, 'b> {
    hello: &'a str,
    world: &'b str,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("{}", self.hello);
    }
}

fn kung_foo<'a, 'b>(_a: &'a Foo, _b: &Foo) -> &'a String {
    &_a.world
}

fn main() {
    let x = Foo {
        hello: "x".to_string(),
        world: "world".to_string(),
    };
    let field = {
        let y = Foo {
            hello: "y".to_string(),
            world: "world".to_string(),
        };
        kung_foo(&x, &y)
    };

    let field = {
        let hello = String::from("hello");
        let x = Bar {
            hello: hello.as_str(),
            world: "world",
        };
        x.world
    };
}
