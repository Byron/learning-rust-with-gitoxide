fn add_one_in_place(n: &mut u32) {
    *n += 1;
}
fn print_int(x: &u32) {
    println!("{}", x);
}
fn main() {
    let x: &mut u32 = &mut 42;
    add_one_in_place(x);
    print_int(x);
}

