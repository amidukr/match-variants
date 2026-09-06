use match_variants::{match_variants, MatchVariants};

pub mod domain {
    use super::MatchVariants;

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

    #[derive(MatchVariants)]
    pub enum NamedValue2 {
        Foo { value: Foo },
        Bar { value: Bar },
    }
}

mod consumer {
    use match_variants::import_match_variants;

    use super::domain;
    use super::match_variants;

    import_match_variants!(
        crate::domain::{NamedValue, TupleValue}
    );

    pub fn tuple_value(value: domain::TupleValue) -> f64 {
        match_variants!(crate::domain::TupleValue, value, (x), { x.value() })
    }

    pub fn named_value(value: domain::NamedValue) -> f64 {
        match_variants!(crate::domain::NamedValue, value, { value: x }, { x.value() })
    }
}

#[test]
fn tuple_enum_can_be_matched_from_another_module() {
    let foo = domain::TupleValue::Foo(domain::Foo(10.0));
    let bar = domain::TupleValue::Bar(domain::Bar(10.0));

    assert_eq!(consumer::tuple_value(foo), 10.0);
    assert_eq!(consumer::tuple_value(bar), 20.0);
}

#[test]
fn named_enum_can_be_matched_from_another_module() {
    let foo = domain::NamedValue::Foo {
        value: domain::Foo(10.0),
    };
    let bar = domain::NamedValue::Bar {
        value: domain::Bar(10.0),
    };

    assert_eq!(consumer::named_value(foo), 10.0);
    assert_eq!(consumer::named_value(bar), 20.0);
}
