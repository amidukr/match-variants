use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value<T> {
    #[variant_type(Vec<T>)]
    Foo(T),

    #[variant_type(Option<T>)]
    Bar(T),
}

fn describe<T>(value: String) -> (&'static str, String) {
    (std::any::type_name::<T>(), value)
}

fn main() {
    let value = Value::Foo(String::from("foo"));

    let _ = match_variants!(Value, value, type Meta, (x), {
        describe::<Meta>(x)
    });
}
