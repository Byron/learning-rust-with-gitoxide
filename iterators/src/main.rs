use std::string::FromUtf8Error;

fn main() -> Result<(), FromUtf8Error> {
    let v = [&b"1"[..], &*b"22", b"1"];
    v.into_iter()
        .filter_map(|v| String::from_utf8(v.into()).ok());
    Ok(())
}
