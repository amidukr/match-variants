#[test]
fn variant_type_compile_failures() {
    let tests = trybuild::TestCases::new();

    tests.compile_fail("tests/ui/empty_with_pattern.rs");
    tests.compile_fail("tests/ui/unit_with_pattern.rs");
    tests.compile_fail("tests/ui/mixed_variant_kinds.rs");

    tests.compile_fail("tests/ui/variant_type_duplicate.rs");
    tests.compile_fail("tests/ui/variant_type_missing.rs");
    tests.compile_fail("tests/ui/variant_type_partial.rs");
    tests.compile_fail("tests/ui/unit_variant_type_missing.rs");
}
