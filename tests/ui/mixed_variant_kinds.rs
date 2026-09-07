use match_variants::MatchVariants;

#[derive(MatchVariants)]
enum Value {
    Foo,
    Bar(f64),
}

fn main() {}
