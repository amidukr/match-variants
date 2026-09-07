use match_variants::{match_variants, MatchVariants};

struct Foo;
struct Bar;

fn type_name<T>() -> &'static str {
    std::any::type_name::<T>()
}

#[derive(MatchVariants)]
enum Value {
    #[variant_type(Foo)]
    Foo,

    #[variant_type(Bar)]
    Bar,
}

#[test]
fn supports_variant_type_for_unit_variants() {
    let value = Value::Foo;

    let result = match_variants!(Value, value, type T, {
        type_name::<T>()
    });

    assert_eq!(result, std::any::type_name::<Foo>());

    let value = Value::Bar;

    let result = match_variants!(Value, value, type T, {
        type_name::<T>()
    });

    assert_eq!(result, std::any::type_name::<Bar>());
}
