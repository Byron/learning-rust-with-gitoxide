fn main() {
    let mut v = vec![0];
    let iter = v.iter();
    v.push(42);
    for item in iter {
        println!("{}", item);
    }
}
