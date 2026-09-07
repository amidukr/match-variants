use match_variants::{match_variants, MatchVariants};

struct Foo;
struct Bar;

fn process_value(value: f64) -> f64 {
    value * 2.0
}

fn process_typed<T>(value: f64) -> (&'static str, f64) {
    (std::any::type_name::<T>(), value)
}

#[derive(MatchVariants)]
enum WithoutMetadata {
    Foo(f64),
    Bar(f64),
}

#[derive(MatchVariants)]
enum WithMetadata {
    #[variant_type(Foo)]
    Foo(f64),

    #[variant_type(Bar)]
    Bar(f64),
}

#[test]
fn works_without_metadata_and_without_type_binding() {
    let value = WithoutMetadata::Foo(21.0);

    let result = match_variants!(WithoutMetadata, value, (x), { process_value(x) });

    assert_eq!(result, 42.0);

    let value = WithoutMetadata::Bar(12.0);

    let result = match_variants!(WithoutMetadata, value, (x), { process_value(x) });

    assert_eq!(result, 24.0);
}

#[test]
fn works_with_complete_metadata_without_type_binding() {
    let value = WithMetadata::Foo(21.0);

    let result = match_variants!(WithMetadata, value, (x), { process_value(x) });

    assert_eq!(result, 42.0);

    let value = WithMetadata::Bar(12.0);

    let result = match_variants!(WithMetadata, value, (x), { process_value(x) });

    assert_eq!(result, 24.0);
}

#[test]
fn works_with_complete_metadata_and_type_binding() {
    let value = WithMetadata::Foo(42.0);

    let result = match_variants!(
        WithMetadata,
        value,
        type T,
        (x),
        {
            process_typed::<T>(x)
        }
    );

    assert_eq!(result, (std::any::type_name::<Foo>(), 42.0));

    let value = WithMetadata::Bar(24.0);

    let result = match_variants!(
        WithMetadata,
        value,
        type T,
        (x),
        {
            process_typed::<T>(x)
        }
    );

    assert_eq!(result, (std::any::type_name::<Bar>(), 24.0));
}
