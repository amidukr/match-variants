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
enum Value {
    Foo(Foo),
    Bar(Bar),
}

fn empty_method() {}

#[test]
fn supports_heterogeneous_payload_types() {
    let foo = Value::Foo(Foo(10.0));
    let bar = Value::Bar(Bar(10.0));

    let foo = match_variants!(Value, foo, (x), {
        empty_method();
        x.value()
    });

    let bar = match_variants!(Value, bar, (x), x.value());

    assert_eq!(foo, 10.0);
    assert_eq!(bar, 20.0);
}
