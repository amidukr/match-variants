use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value {
    Foo {},
    Bar {},
}

fn main() {
    let value = Value::Foo {};

    let _ = match_variants!(Value, value, {}, 42);
}
