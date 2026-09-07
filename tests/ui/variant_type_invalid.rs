use match_variants::MatchVariants;

struct Foo;
struct Bar;

#[derive(MatchVariants)]
enum Value {
    #[variant_type(Foo, Bar)]
    Foo,
}

fn main() {}
