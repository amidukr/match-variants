#[test]
fn variant_type_compile_failures() {
    let tests = trybuild::TestCases::new();

    tests.compile_fail("tests/ui/empty_with_pattern.rs");
    tests.compile_fail("tests/ui/non_enum.rs");
    tests.compile_fail("tests/ui/unit_with_pattern.rs");
    tests.compile_fail("tests/ui/mixed_variant_kinds.rs");

    tests.compile_fail("tests/ui/match_variants_invalid_named_pattern.rs");
    tests.compile_fail("tests/ui/match_variants_invalid_tuple_pattern.rs");
    tests.compile_fail("tests/ui/match_variants_type_missing_comma.rs");

    tests.compile_fail("tests/ui/empty_variant_type_missing.rs");
    tests.compile_fail("tests/ui/variant_type_duplicate.rs");
    tests.compile_fail("tests/ui/variant_type_invalid.rs");
    tests.compile_fail("tests/ui/variant_type_generic_param.rs");
    tests.compile_fail("tests/ui/variant_type_missing.rs");
    tests.compile_fail("tests/ui/variant_type_partial.rs");
    tests.compile_fail("tests/ui/unit_variant_type_missing.rs");
}
