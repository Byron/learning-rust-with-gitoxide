#![allow(unused)]

struct A;
struct B;
struct C;

impl From<B> for A {
    fn from(_: B) -> Self {
        A
    }
}
impl From<C> for A {
    fn from(_: C) -> Self {
        A
    }
}

fn foo(a: impl Into<A>) {
    let a = a.into();
}
fn foo_b(a: B) {
    foo_inner(a.into())
}
fn foo_c(a: C) {
    foo_inner(a.into())
}
fn foo_inner(a: A) {
    // millions of lines
}

enum Z {
    A,
    ParsedC,
}

impl From<A> for Z {
    fn from(_: A) -> Self {
        Z::A
    }
}

impl TryFrom<C> for Z {
    type Error = ();

    fn try_from(value: C) -> Result<Self, Self::Error> {
        Err(())
    }
}

fn main() {
    let a: A = B.into();
    let a = Into::<A>::into(B);
    let a = A::from(B);
    let a = A::from(C);
    let a: A = C.into();

    let x: u32 = "8".parse().unwrap();
    let z: Z = A.into();
    let z: Z = A.try_into().unwrap();
    let z: Z = C.try_into().unwrap();

    foo(B);
    foo(C);
    foo(A);
}
