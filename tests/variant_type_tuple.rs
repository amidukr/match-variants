use match_variants::{match_variants, MatchVariants};

struct Foo;
struct Bar;

#[derive(MatchVariants)]
enum ValueWithPayload {
    #[variant_type(Foo)]
    Foo(f64),

    #[variant_type(Bar)]
    Bar(f64),
}

fn process<T>(value: f64) -> (&'static str, f64) {
    (std::any::type_name::<T>(), value)
}

fn process_type<T>() -> &'static str {
    std::any::type_name::<T>()
}

#[test]
fn supports_variant_type_with_tuple_payload_and_pattern() {
    let value = ValueWithPayload::Foo(42.0);

    let result = match_variants!(
        ValueWithPayload,
        value,
        type T,
        (x),
        {
            process::<T>(x)
        }
    );

    assert_eq!(result, (std::any::type_name::<Foo>(), 42.0));

    let value = ValueWithPayload::Bar(24.0);

    let result = match_variants!(
        ValueWithPayload,
        value,
        type T,
        (x),
        {
            process::<T>(x)
        }
    );

    assert_eq!(result, (std::any::type_name::<Bar>(), 24.0));
}

#[test]
fn supports_variant_type_with_tuple_payload_without_pattern() {
    let value = ValueWithPayload::Foo(42.0);

    let result = match_variants!(
        ValueWithPayload,
        value,
        type T,
        {
            process_type::<T>()
        }
    );

    assert_eq!(result, std::any::type_name::<Foo>());

    let value = ValueWithPayload::Bar(24.0);

    let result = match_variants!(
        ValueWithPayload,
        value,
        type T,
        {
            process_type::<T>()
        }
    );

    assert_eq!(result, std::any::type_name::<Bar>());
}

#[test]
fn supports_tuple_payload_with_pattern_without_type_binding() {
    let value = ValueWithPayload::Foo(42.0);

    let result = match_variants!(ValueWithPayload, value, (x), x);

    assert_eq!(result, 42.0);

    let value = ValueWithPayload::Bar(24.0);

    let result = match_variants!(ValueWithPayload, value, (x), x);

    assert_eq!(result, 24.0);
}

#[test]
fn supports_tuple_payload_without_pattern_or_type_binding() {
    let value = ValueWithPayload::Foo(42.0);

    let result = match_variants!(ValueWithPayload, value, 1);

    assert_eq!(result, 1);

    let value = ValueWithPayload::Bar(24.0);

    let result = match_variants!(ValueWithPayload, value, 2);

    assert_eq!(result, 2);
}
