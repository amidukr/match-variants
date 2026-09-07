#![allow(unused_imports)]

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

mod domain {
    use super::MatchVariants;

    pub struct Foo;
    pub struct Bar;

    impl Foo {
        pub fn value(self) -> i32 {
            30
        }
    }

    impl Bar {
        pub fn value(self) -> i32 {
            40
        }
    }

    #[derive(MatchVariants)]
    pub enum LocalEnum {
        Foo(Foo),
        Bar(Bar),
    }
}

#[test]
fn supports_two_segment_local_module_enum_path() {
    use domain::LocalEnum;

    let foo = domain::LocalEnum::Foo(domain::Foo);
    let bar = domain::LocalEnum::Bar(domain::Bar);

    let foo_result = match_variants!(domain::LocalEnum, foo, (x), x.value());
    let bar_result = match_variants!(domain::LocalEnum, bar, (x), x.value());

    assert_eq!(foo_result, 30);
    assert_eq!(bar_result, 40);
}
