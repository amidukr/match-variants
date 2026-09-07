#![allow(clippy::crate_in_macro_def)]

use match_variants::{match_variants, MatchVariants};

mod types {
    pub struct Foo;
    pub struct Bar;
    pub struct Wrapper<T>(pub T);
}

#[derive(MatchVariants)]
enum Value {
    #[variant_type(crate::types::Foo)]
    Foo,

    #[variant_type(crate::types::Wrapper<crate::types::Bar>)]
    Bar,
}

fn type_name<T>() -> &'static str {
    std::any::type_name::<T>()
}

#[test]
fn variant_type_accepts_paths_and_generic_types() {
    let value = Value::Foo;

    let result = match_variants!(Value, value, type T, {
        type_name::<T>()
    });

    assert_eq!(result, std::any::type_name::<types::Foo>());

    let value = Value::Bar;

    let result = match_variants!(Value, value, type T, {
        type_name::<T>()
    });

    assert_eq!(result, std::any::type_name::<types::Wrapper<types::Bar>>());
}
