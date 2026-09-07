use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value {
    Foo,
    Bar,
}

fn consume(value: Value) -> i32 {
    match_variants!(Value, value, 42)
}

#[test]
fn supports_unit_variants() {
    assert_eq!(consume(Value::Foo), 42);
    assert_eq!(consume(Value::Bar), 42);
}
