#[test]
fn ownership() {
    let t = trybuild::TestCases::new();
    t.compile_fail("fail/vec-push.rs");
    t.pass("pass/borrow-mut-int.rs");
    t.pass("pass/copy-clone-unit-struct.rs");
}