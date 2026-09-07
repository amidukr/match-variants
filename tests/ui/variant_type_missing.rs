use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value {
    Foo(f64),
    Bar(f64),
}

fn process<T>(value: f64) -> f64 {
    value
}

fn main() {
    let value = Value::Foo(42.0);

    let _ = match_variants!(
        Value,
        value,
        type T,
        (x),
        {
            process::<T>(x)
        }
    );
}
