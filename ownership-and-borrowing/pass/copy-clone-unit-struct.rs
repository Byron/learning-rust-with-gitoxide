#[derive(Copy, Clone)]
struct Foo;

fn kung_foo(_fighter: Foo) {
}

fn main() {
    let x = Foo;
    kung_foo(x.clone());
    kung_foo(x);
}