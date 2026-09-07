use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Empty {}

fn process<T>() -> &'static str {
    std::any::type_name::<T>()
}

fn consume(value: Empty) -> &'static str {
    match_variants!(Empty, value, type T, {
        process::<T>()
    })
}

fn main() {}
