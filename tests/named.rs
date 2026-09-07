use match_variants::{match_variants, MatchVariants};

struct Foo(f64);
struct Bar(f64);

impl Foo {
    fn value(self) -> f64 {
        self.0
    }
}

impl Bar {
    fn value(self) -> f64 {
        self.0 * 2.0
    }
}

#[derive(MatchVariants)]
enum SingleFieldNamed {
    Foo { value: Foo },
    Bar { value: Bar },
}

fn empty_method() {}

#[test]
fn named_single_field_supports_heterogeneous_types() {
    let foo = SingleFieldNamed::Foo { value: Foo(10.0) };

    let bar = SingleFieldNamed::Bar { value: Bar(10.0) };

    let foo_result = match_variants!(
        SingleFieldNamed,
        foo,
        { value: x },
        {
            empty_method();
            x.value()
        }
    );

    let bar_result = match_variants!(
        SingleFieldNamed,
        bar,
        { value: x },
        x.value()
    );

    assert_eq!(foo_result, 10.0);
    assert_eq!(bar_result, 20.0);
}

struct FooContext(f64);
struct BarContext(f64);

impl Foo {
    fn process(self, ctx: FooContext) -> f64 {
        self.0 + ctx.0
    }
}

impl Bar {
    fn process(self, ctx: BarContext) -> f64 {
        self.0 * ctx.0
    }
}

#[derive(MatchVariants)]
enum MultiFieldNamed {
    Foo { value: Foo, context: FooContext },
    Bar { value: Bar, context: BarContext },
}

#[test]
fn named_multiple_fields_are_bound() {
    let foo = MultiFieldNamed::Foo {
        value: Foo(10.0),
        context: FooContext(5.0),
    };

    let bar = MultiFieldNamed::Bar {
        value: Bar(10.0),
        context: BarContext(5.0),
    };

    let foo_result = match_variants!(
        MultiFieldNamed,
        foo,
        {
            value: x,
            context: ctx
        },
        {
            empty_method();
            x.process(ctx)
        }
    );

    let bar_result = match_variants!(
        MultiFieldNamed,
        bar,
        {
            value: x,
            context: ctx
        },
        x.process(ctx)
    );

    assert_eq!(foo_result, 15.0);
    assert_eq!(bar_result, 50.0);
}
#[test]
fn named_variants_can_be_matched_without_binding_payload() {
    let foo = SingleFieldNamed::Foo { value: Foo(10.0) };
    let bar = SingleFieldNamed::Bar { value: Bar(10.0) };

    let foo_result = match_variants!(SingleFieldNamed, foo, "named");
    let bar_result = match_variants!(SingleFieldNamed, bar, "named");

    assert_eq!(foo_result, "named");
    assert_eq!(bar_result, "named");
}
