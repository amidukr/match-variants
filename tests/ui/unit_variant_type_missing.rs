use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value {
    Foo,
    Bar,
}

fn consume(value: Value) {
    match_variants!(Value, value, type T, {
        let _: Option<T> = None;
    })
}

fn main() {}
