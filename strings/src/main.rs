fn main() {
    let s: &'static str = "hello😅👋🏼👋🏿你好";
    for char in s.chars() {
        println!("{}", char)
    }
    println!(
        "{} = sizeof(char) ({} = MAX chars, unlike {} u32 MAX)",
        std::mem::size_of::<char>(),
        char::MAX as u32,
        u32::MAX
    );
    println!("{:?}", &s[..9]);
    s.to_owned().push('c');
    s.to_owned().push_str("c");
}
