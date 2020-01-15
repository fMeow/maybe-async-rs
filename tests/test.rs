#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01-maybe-async.rs");
    t.pass("tests/ui/02-must-be-async.rs");
    t.pass("tests/ui/03-must-be-sync.rs");
}
