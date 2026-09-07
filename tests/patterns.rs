use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum TuplePatterns {
    A(i32, String, (i32, i32)),
    B(i32, String, (i32, i32)),
}

#[test]
fn tuple_patterns_support_ref_mut_nested_and_trailing_commas() {
    let value = TuplePatterns::A(10, String::from("foo"), (2, 3));

    let result = match_variants!(
        TuplePatterns,
        value,
        (ref number, mut text, (left, right)),
        {
            text.push('!');

            *number + text.len() as i32 + left + right
        }
    );

    assert_eq!(result, 19);
}

#[test]
fn tuple_patterns_support_wildcards_and_at_bindings() {
    let value = TuplePatterns::B(7, String::from("ignored"), (4, 5));

    let result = match_variants!(TuplePatterns, value, (_, _, pair @ (_, _)), {
        pair.0 + pair.1
    });

    assert_eq!(result, 9);
}

#[derive(MatchVariants)]
enum NamedPatterns {
    A { value: i32, label: &'static str },
    B { value: i32, label: &'static str },
}

#[test]
fn named_patterns_support_ref_at_wildcards_and_trailing_commas() {
    let value = NamedPatterns::A {
        value: 42,
        label: "ready",
    };

    #[allow(clippy::redundant_pattern)]
    let result = match_variants!(
        NamedPatterns,
        value,
        {
            value: ref number,
            label: name @ _,
        },
        {
            (*number, name)
        }
    );

    assert_eq!(result, (42, "ready"));

    let value = NamedPatterns::B {
        value: 0,
        label: "skip",
    };

    let result = match_variants!(NamedPatterns, value, { value: _, label: name }, name);

    assert_eq!(result, "skip");
}
