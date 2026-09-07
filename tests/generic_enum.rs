use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value<T> {
    Foo(T),
    Bar(T),
}

#[test]
fn supports_generic_enum_payloads() {
    let foo = Value::Foo(String::from("foo"));
    let bar = Value::Bar(String::from("bar"));

    let foo_result = match_variants!(Value, foo, (x), x.len());
    let bar_result = match_variants!(Value, bar, (x), x.len());

    assert_eq!(foo_result, 3);
    assert_eq!(bar_result, 3);
}
