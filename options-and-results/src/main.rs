#![allow(unused, dead_code)]

fn add_one_safely(v: u8) -> Result<u8, &'static str> {
    v.checked_add(1).ok_or("overflow")
}

fn main() -> Result<(), &'static str> {
    std::env::args()
        .is_empty()
        .then(|| "need two args")
        .map(Err)
        .transpose()?;
    println!("{}", add_one_safely(255)?);
    Ok(())
}
