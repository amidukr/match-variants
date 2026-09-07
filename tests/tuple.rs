use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Numbers {
    A(f64),
    B(f64),
}

#[test]
fn applies_expression_to_all_variants() {
    let a = Numbers::A(5.0);
    let b = Numbers::B(8.0);

    let a = match_variants!(Numbers, a, (x), x + 1.0);
    let b = match_variants!(Numbers, b, (x), x + 1.0);

    assert_eq!(a, 6.0);
    assert_eq!(b, 9.0);
}

#[test]
fn tuple_variants_can_be_matched_without_binding_payload() {
    let a = Numbers::A(5.0);
    let b = Numbers::B(8.0);

    let a = match_variants!(Numbers, a, "number");
    let b = match_variants!(Numbers, b, "number");

    assert_eq!(a, "number");
    assert_eq!(b, "number");
}

#[derive(MatchVariants)]
enum Pair {
    A(f64, f64),
    B(f64, f64),
}

#[test]
fn tuple_multiple_fields_are_bound() {
    let a = Pair::A(5.0, 2.0);
    let b = Pair::B(8.0, 3.0);

    let a = match_variants!(Pair, a, (x, y), x + y);
    let b = match_variants!(Pair, b, (x, y), x * y);

    assert_eq!(a, 7.0);
    assert_eq!(b, 24.0);
}

#[derive(MatchVariants)]
enum EmptyTupleVariant {
    Foo(),
    Bar(),
}

#[test]
fn empty_tuple_variants_can_be_matched_without_binding_payload() {
    let foo = EmptyTupleVariant::Foo();
    let bar = EmptyTupleVariant::Bar();

    let foo_result = match_variants!(EmptyTupleVariant, foo, "tuple");
    let bar_result = match_variants!(EmptyTupleVariant, bar, "tuple");

    assert_eq!(foo_result, "tuple");
    assert_eq!(bar_result, "tuple");
}

#[test]
fn empty_tuple_variants_accept_empty_tuple_pattern() {
    let foo = EmptyTupleVariant::Foo();
    let bar = EmptyTupleVariant::Bar();

    let foo_result = match_variants!(EmptyTupleVariant, foo, (), "foo");
    let bar_result = match_variants!(EmptyTupleVariant, bar, (), "bar");

    assert_eq!(foo_result, "foo");
    assert_eq!(bar_result, "bar");
}
