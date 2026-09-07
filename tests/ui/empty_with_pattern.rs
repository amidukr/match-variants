use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Empty {}

fn consume(value: Empty) -> i32 {
    match_variants!(Empty, value, (x), {
        let _ = x;
        42
    })
}

fn main() {}
