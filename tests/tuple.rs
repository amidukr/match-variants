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
fn applies_expression_to_all_variants_with_expression() {
    let a = Numbers::A(5.0);
    let b = Numbers::B(8.0);

    let a = match_variants!(Numbers, a, (x), x + 1.0);
    let b = match_variants!(Numbers, b, (x), x + 1.0);

    assert_eq!(a, 6.0);
    assert_eq!(b, 9.0);
}
