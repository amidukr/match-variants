use match_variants::{match_variants, MatchVariants};

struct Foo;
struct Bar;

#[derive(MatchVariants)]
enum ValueWithPayload {
    #[variant_type(Foo)]
    Foo { value: f64 },

    #[variant_type(Bar)]
    Bar { value: f64 },
}

fn process<T>(value: f64) -> (&'static str, f64) {
    (std::any::type_name::<T>(), value)
}

#[test]
fn supports_variant_type_with_named_payload() {
    let value = ValueWithPayload::Foo { value: 42.0 };

    let result = match_variants!(
        ValueWithPayload,
        value,
        type T,
        { value: x },
        {
            process::<T>(x)
        }
    );

    assert_eq!(result, (std::any::type_name::<Foo>(), 42.0));

    let value = ValueWithPayload::Bar { value: 24.0 };

    let result = match_variants!(
        ValueWithPayload,
        value,
        type T,
        { value: x },
        {
            process::<T>(x)
        }
    );

    assert_eq!(result, (std::any::type_name::<Bar>(), 24.0));
}
