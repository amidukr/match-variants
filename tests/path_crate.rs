use match_variants::{match_variants, MatchVariants};

struct Foo;
struct Bar;

impl Foo {
    fn value(self) -> i32 {
        10
    }
}

impl Bar {
    fn value(self) -> i32 {
        20
    }
}

#[derive(MatchVariants)]
enum MyEnum {
    Foo(Foo),
    Bar(Bar),
}

#[test]
fn supports_crate_qualified_enum_path() {
    let foo = crate::MyEnum::Foo(Foo);
    let bar = crate::MyEnum::Bar(Bar);

    let foo_result = match_variants!(crate::MyEnum, foo, (x), x.value());

    let bar_result = match_variants!(crate::MyEnum, bar, (x), x.value());

    assert_eq!(foo_result, 10);
    assert_eq!(bar_result, 20);
}
