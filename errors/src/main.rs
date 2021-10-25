#[derive(Debug)]
enum OutOfBoundsError {
    Overflow,
    Underflow,
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

fn main() -> Result<(), ()> {
    match Adder(&126).add_one() {
        Ok(v) => {}
        Err(v) => return,
    }
    Adder(&-128).sub_one().unwrap();
    Ok(())
}
