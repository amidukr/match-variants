use match_variants::MatchVariants;

struct Foo;
struct Bar;

#[derive(MatchVariants)]
enum Value {
    #[variant_type(Foo)]
    #[variant_type(Bar)]
    Foo,
}

fn main() {}
