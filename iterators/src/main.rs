fn main() {
    let mut v = [String::from("1"), "2".into(), "3".into()];
    let mut iter = v.iter();
    let mut iter_ref = iter.by_ref().filter(|s| *s == "2");
    dbg!(iter_ref.next());
    dbg!(iter.next());
}
