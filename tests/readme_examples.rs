#![allow(dead_code, unused_imports)]

mod intro {
    use match_variants::{match_variants, MatchVariants};

    struct Foo(f64);
    struct Bar(f64);

    impl Foo {
        fn value(self) -> f64 {
            self.0
        }
    }

    impl Bar {
        fn value(self) -> f64 {
            self.0 * 2.0
        }
    }

    #[derive(MatchVariants)]
    enum IntroValue {
        Foo(Foo),
        Bar(Bar),
    }

    fn value(value: IntroValue) -> f64 {
        match_variants!(IntroValue, value, (x), x.value())
    }

    #[test]
    fn works() {
        assert_eq!(value(IntroValue::Foo(Foo(10.0))), 10.0);
        assert_eq!(value(IntroValue::Bar(Bar(10.0))), 20.0);
    }
}

mod tuple_variant_type_payload {
    use match_variants::{match_variants, MatchVariants};

    struct Foo;
    struct Bar;

    #[derive(MatchVariants)]
    enum TuplePayloadValue {
        #[variant_type(Foo)]
        Foo(f64),

        #[variant_type(Bar)]
        Bar(f64),
    }

    fn process<T>(value: f64) -> (&'static str, f64) {
        (std::any::type_name::<T>(), value)
    }

    #[test]
    fn works() {
        let value = TuplePayloadValue::Foo(42.0);

        let result = match_variants!(
            TuplePayloadValue,
            value,
            type T,
            (x),
            {
                process::<T>(x)
            }
        );

        assert_eq!(result, (std::any::type_name::<Foo>(), 42.0));

        let value = TuplePayloadValue::Bar(24.0);

        let result = match_variants!(
            TuplePayloadValue,
            value,
            type T,
            (x),
            {
                process::<T>(x)
            }
        );

        assert_eq!(result, (std::any::type_name::<Bar>(), 24.0));
    }
}

mod named_variant_type_payload {
    use match_variants::{match_variants, MatchVariants};

    struct Foo;
    struct Bar;

    fn process<T>(value: f64) -> (&'static str, f64) {
        (std::any::type_name::<T>(), value)
    }

    #[derive(MatchVariants)]
    enum NamedPayloadValue {
        #[variant_type(Foo)]
        Foo { value: f64 },

        #[variant_type(Bar)]
        Bar { value: f64 },
    }

    #[test]
    fn works() {
        let value = NamedPayloadValue::Foo { value: 42.0 };

        let result = match_variants!(
            NamedPayloadValue,
            value,
            type T,
            { value: x },
            {
                process::<T>(x)
            }
        );

        assert_eq!(result, (std::any::type_name::<Foo>(), 42.0));

        let value = NamedPayloadValue::Bar { value: 24.0 };

        let result = match_variants!(
            NamedPayloadValue,
            value,
            type T,
            { value: x },
            {
                process::<T>(x)
            }
        );

        assert_eq!(result, (std::any::type_name::<Bar>(), 24.0));
    }
}

mod module_example {
    pub mod domain {
        use match_variants::MatchVariants;

        pub struct Foo(pub f64);
        pub struct Bar(pub f64);

        impl Foo {
            pub fn value(self) -> f64 {
                self.0
            }
        }

        impl Bar {
            pub fn value(self) -> f64 {
                self.0 * 2.0
            }
        }

        #[derive(MatchVariants)]
        pub enum TupleValue {
            Foo(Foo),
            Bar(Bar),
        }

        #[derive(MatchVariants)]
        pub enum NamedValue {
            Foo { value: Foo },
            Bar { value: Bar },
        }
    }

    mod consumer {
        use match_variants::{import_match_variants, match_variants};

        use super::domain;

        import_match_variants!(
            crate::module_example::domain::{
                NamedValue,
                TupleValue,
            }
        );

        pub fn tuple_value(value: domain::TupleValue) -> f64 {
            match_variants!(crate::module_example::domain::TupleValue, value, (x), {
                x.value()
            })
        }

        pub fn named_value(value: domain::NamedValue) -> f64 {
            match_variants!(
                crate::module_example::domain::NamedValue,
                value,
                { value: x },
                {
                    x.value()
                }
            )
        }
    }

    #[test]
    fn works() {
        let tuple = domain::TupleValue::Bar(domain::Bar(10.0));
        let named = domain::NamedValue::Foo {
            value: domain::Foo(10.0),
        };

        assert_eq!(consumer::tuple_value(tuple), 20.0);
        assert_eq!(consumer::named_value(named), 10.0);
    }
}
