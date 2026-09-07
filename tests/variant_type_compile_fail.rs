#[test]
fn variant_type_compile_failures() {
    let tests = trybuild::TestCases::new();

    tests.compile_fail("tests/ui/variant_type_partial.rs");
    tests.compile_fail("tests/ui/variant_type_missing.rs");
}
