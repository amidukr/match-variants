use match_variants::{match_variants, MatchVariants};

struct Foo;
struct Bar;

#[derive(MatchVariants)]
enum Value {
    #[variant_type(Foo)]
    Foo,

    #[variant_type(Bar)]
    Bar,
}

fn main() {
    let value = Value::Foo;

    let _ = match_variants!(Value, value, type T {
        std::any::type_name::<T>()
    });
}
