//! Tests for compile-time macro errors using trybuild

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/macro_error_tests/ui/*.rs");
}

