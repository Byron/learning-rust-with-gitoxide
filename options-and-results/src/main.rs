#![allow(unused, dead_code)]

enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    fn take_map<U>(&mut self, f: impl FnOnce(T) -> U) -> Option<U> {
        match self.take() {
            MyOption::Some(v) => Some(f(v)),
            MyOption::None => None,
        }
    }

    fn take(&mut self) -> MyOption<T> {
        std::mem::replace(self, MyOption::None)
    }
}

fn change(mut v: Option<&mut String>) {
    v.map(|v: &mut String| *v = String::from("changed"));
}

fn main() {
    let mut v = vec![String::from("hello")];
    change(v.get_mut(0));
    println!("{:?}", v.get(0));
}
