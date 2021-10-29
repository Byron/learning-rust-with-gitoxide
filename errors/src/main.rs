use anyhow::Context;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum OutOfBoundsError {
    Overflow,
    Underflow,
}

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for OutOfBoundsError {}

impl From<OutOfBoundsError> for String {
    fn from(v: OutOfBoundsError) -> Self {
        format!("{:?}", v)
    }
}

struct Adder<'a>(&'a i8);

impl<'a> Adder<'a> {
    fn add_one(&self) -> Result<i8, OutOfBoundsError> {
        self.0.checked_add(1).ok_or(OutOfBoundsError::Overflow)
    }

    fn sub_one(&self) -> Result<i8, OutOfBoundsError> {
        self.0.checked_sub(1).ok_or(OutOfBoundsError::Underflow)
    }
}

fn always_err() -> anyhow::Result<()> {
    // anyhow::bail!("hello, failure")
    Err(anyhow::anyhow!("hello too"))
}

fn do_it() -> anyhow::Result<()> {
    let v = Adder(&126).add_one()?;
    always_err().context("additional information")?;
    println!("{}", v);
    Adder(&-128).sub_one()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    do_it()?;
    Ok(())
}
