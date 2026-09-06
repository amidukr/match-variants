use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Heterogeneous {
    Number(f64),
    Text(String),
}

#[test]
fn same_body_can_compile_for_different_concrete_types() {
    let number = Heterogeneous::Number(12.5);
    let text = Heterogeneous::Text("hello".to_string());

    let number_result = match_variants!(Heterogeneous, number, (x), { x.to_string() });

    let text_result = match_variants!(Heterogeneous, text, (x), { x.to_string() });

    assert_eq!(number_result, "12.5");
    assert_eq!(text_result, "hello");
}
