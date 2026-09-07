use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value {
    Foo,
    Bar,
}

fn consume(value: Value) -> i32 {
    match_variants!(Value, value, (x), {
        let _ = x;
        42
    })
}

fn main() {}
