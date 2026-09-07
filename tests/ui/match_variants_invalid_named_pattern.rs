use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value {
    Foo { value: i32 },
    Bar { value: i32 },
}

fn main() {
    let value = Value::Foo { value: 42 };

    let _ = match_variants!(Value, value, { value x }, x);
}
