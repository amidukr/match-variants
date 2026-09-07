use match_variants::{match_variants, MatchVariants};

#[derive(MatchVariants)]
enum Value {
    Foo(i32),
    Bar(i32),
}

fn main() {
    let value = Value::Foo(42);

    let _ = match_variants!(Value, value, (x,),, x);
}
