use match_variants::{match_variants, MatchVariants};

struct Foo<T> {
    value: T,
}

struct Bar<T> {
    value: T,
}

trait Value {
    fn value(&self) -> i32;
}

trait Marker {}

impl Value for Foo<i32> {
    fn value(&self) -> i32 {
        self.value
    }
}

impl Value for Bar<i32> {
    fn value(&self) -> i32 {
        self.value
    }
}

#[allow(unused)]
#[derive(MatchVariants)]
enum Kind {
    #[variant_type(Foo<U>)]
    Foo,

    #[variant_type(Bar<U>)]
    Bar,
}

#[allow(type_alias_bounds)]
#[test]
fn supports_generic_type_binding_with_constraint() {
    let kind = Kind::Foo;

    let result = match_variants!(
        Kind,
        kind,
        type T<U: Marker>,
        {
            let value = T { value: 123 };
            value.value()
        }
    );

    assert_eq!(result, 123);
}
