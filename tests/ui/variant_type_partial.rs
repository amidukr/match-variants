use match_variants::MatchVariants;

struct Foo;

#[derive(MatchVariants)]
enum Value {
    #[variant_type(Foo)]
    Foo(f64),

    Bar(f64),
}

fn main() {}
