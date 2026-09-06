use match_variants::match_variants;

pub mod foo {
    pub mod bar {
        use match_variants::MatchVariants;

        pub struct Foo;
        pub struct Bar;

        impl Foo {
            pub fn value(self) -> i32 {
                10
            }
        }

        impl Bar {
            pub fn value(self) -> i32 {
                20
            }
        }

        #[derive(MatchVariants)]
        pub enum MyEnum {
            Foo(Foo),
            Bar(Bar),
        }
    }
}

#[test]
fn supports_nested_crate_qualified_enum_path() {
    let foo = crate::foo::bar::MyEnum::Foo(crate::foo::bar::Foo);

    let bar = crate::foo::bar::MyEnum::Bar(crate::foo::bar::Bar);

    let foo_result = match_variants!(crate::foo::bar::MyEnum, foo, (x), { x.value() });

    let bar_result = match_variants!(crate::foo::bar::MyEnum, bar, (x), { x.value() });

    assert_eq!(foo_result, 10);
    assert_eq!(bar_result, 20);
}
