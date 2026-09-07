//! Match enum variants without requiring their payload types to implement
//! a common trait.
//!
//! `match-variants` generates a small helper macro for an enum and uses it
//! to apply the same expression to every variant.
//!
//! Each generated match arm is independently type-checked against its
//! concrete variant, with no dynamic dispatch or runtime type inspection.
//!
//! The crate also supports associating a type with each enum variant through
//! `#[variant_type(...)]`. That type can be introduced as a local type alias
//! inside each generated match arm.
//!
//! # Matching variant payloads
//!
//! ```
//! use match_variants::{match_variants, MatchVariants};
//!
//! struct Foo(f64);
//! struct Bar(f64);
//!
//! impl Foo {
//!     fn value(self) -> f64 {
//!         self.0
//!     }
//! }
//!
//! impl Bar {
//!     fn value(self) -> f64 {
//!         self.0 * 2.0
//!     }
//! }
//!
//! #[derive(MatchVariants)]
//! enum Value {
//!     Foo(Foo),
//!     Bar(Bar),
//! }
//!
//! fn value(value: Value) -> f64 {
//!     match_variants!(Value, value, (x), {
//!         x.value()
//!     })
//! }
//!
//! assert_eq!(value(Value::Foo(Foo(10.0))), 10.0);
//! assert_eq!(value(Value::Bar(Bar(10.0))), 20.0);
//! ```
//!
//! # Associating types with variants
//!
//! `#[variant_type(...)]` associates a Rust type with an enum variant.
//! `match_variants!` can expose that type through a local type alias:
//!
//! ```
//! use match_variants::{match_variants, MatchVariants};
//!
//! struct Foo;
//! struct Bar;
//!
//! trait TypeName {
//!     const NAME: &'static str;
//! }
//!
//! impl TypeName for Foo {
//!     const NAME: &'static str = "Foo";
//! }
//!
//! impl TypeName for Bar {
//!     const NAME: &'static str = "Bar";
//! }
//!
//! #[derive(MatchVariants)]
//! enum Value {
//!     #[variant_type(Foo)]
//!     Foo,
//!
//!     #[variant_type(Bar)]
//!     Bar,
//! }
//!
//! fn name(value: Value) -> &'static str {
//!     match_variants!(Value, value, type T, {
//!         T::NAME
//!     })
//! }
//!
//! assert_eq!(name(Value::Foo), "Foo");
//! assert_eq!(name(Value::Bar), "Bar");
//! ```
//!
//! Conceptually, the invocation above expands to:
//!
//! ```text
//! match value {
//!     Value::Foo => {
//!         type T = Foo;
//!         T::NAME
//!     }
//!     Value::Bar => {
//!         type T = Bar;
//!         T::NAME
//!     }
//! }
//! ```
//!
//! The associated type is independent of the variant's payload. It can
//! therefore also be used with tuple and struct-like variants.

use proc_macro::TokenStream;

mod derive;
mod import_match_variants;
mod match_variants;

/// Generates the helper macro required by [`match_variants!`].
///
/// `MatchVariants` can be derived for enums whose variants are:
///
/// - all unit variants,
/// - all tuple variants, or
/// - all struct-like variants.
///
/// Mixing different variant shapes in the same enum is not supported.
///
/// # Variant-associated types
///
/// A type can optionally be associated with every variant using
/// `#[variant_type(...)]`:
///
/// ```
/// use match_variants::MatchVariants;
///
/// struct Foo;
/// struct Bar;
///
/// #[derive(MatchVariants)]
/// enum Value {
///     #[variant_type(Foo)]
///     Foo,
///
///     #[variant_type(Bar)]
///     Bar,
/// }
/// ```
///
/// The attribute accepts a Rust type, including paths and generic types.
///
/// If `#[variant_type(...)]` is used, it must be specified on every variant.
/// An enum may therefore have either:
///
/// - no `#[variant_type(...)]` attributes, or
/// - a `#[variant_type(...)]` attribute on every variant.
///
/// Partial variant type metadata is rejected at compile time.
///
/// For an enum named `Value`, the derive generates a helper macro named
/// `match_variants_for_Value`.
#[proc_macro_derive(MatchVariants, attributes(variant_type))]
pub fn derive_match_variants(input: TokenStream) -> TokenStream {
    derive::expand(input)
}

/// Applies the same expression to every variant of an enum.
///
/// Bindings are concrete in each generated match arm. Different variants may
/// therefore contain unrelated types as long as the supplied expression is
/// valid for every generated arm and all arms produce compatible results.
///
/// # Tuple variants
///
/// ```
/// use match_variants::{match_variants, MatchVariants};
///
/// struct Foo(i32);
/// struct Bar(i32);
///
/// impl Foo {
///     fn value(self) -> i32 {
///         self.0
///     }
/// }
///
/// impl Bar {
///     fn value(self) -> i32 {
///         self.0
///     }
/// }
///
/// #[derive(MatchVariants)]
/// enum Value {
///     Foo(Foo),
///     Bar(Bar),
/// }
///
/// let value = Value::Foo(Foo(42));
///
/// let result = match_variants!(Value, value, (x), {
///     x.value()
/// });
///
/// assert_eq!(result, 42);
/// ```
///
/// # Struct-like variants
///
/// ```
/// use match_variants::{match_variants, MatchVariants};
///
/// struct Foo(i32);
/// struct Bar(i32);
///
/// impl Foo {
///     fn value(self) -> i32 {
///         self.0
///     }
/// }
///
/// impl Bar {
///     fn value(self) -> i32 {
///         self.0
///     }
/// }
///
/// #[derive(MatchVariants)]
/// enum Value {
///     Foo { value: Foo },
///     Bar { value: Bar },
/// }
///
/// let value = Value::Bar { value: Bar(42) };
///
/// let result = match_variants!(Value, value, { value: x }, {
///     x.value()
/// });
///
/// assert_eq!(result, 42);
/// ```
///
/// # Unit variants
///
/// Unit variants do not require a payload pattern:
///
/// ```
/// use match_variants::{match_variants, MatchVariants};
///
/// #[derive(MatchVariants)]
/// enum Value {
///     Foo,
///     Bar,
/// }
///
/// let value = Value::Foo;
///
/// let result = match_variants!(Value, value, {
///     42
/// });
///
/// assert_eq!(result, 42);
/// ```
///
/// # Variant-associated type binding
///
/// When every variant has `#[variant_type(...)]`, the associated type can be
/// bound to a local type alias by writing `type T` before the optional payload
/// pattern:
///
/// ```
/// use match_variants::{match_variants, MatchVariants};
///
/// struct Foo;
/// struct Bar;
///
/// fn type_name<T>() -> &'static str {
///     std::any::type_name::<T>()
/// }
///
/// #[derive(MatchVariants)]
/// enum Value {
///     #[variant_type(Foo)]
///     Foo,
///
///     #[variant_type(Bar)]
///     Bar,
/// }
///
/// let value = Value::Foo;
///
/// let result = match_variants!(Value, value, type T, {
///     type_name::<T>()
/// });
///
/// assert_eq!(result, std::any::type_name::<Foo>());
/// ```
///
/// `T` is a local type alias whose concrete type is different in each match
/// arm. The generated code is conceptually equivalent to:
///
/// ```text
/// match value {
///     Value::Foo => {
///         type T = Foo;
///         type_name::<T>()
///     }
///     Value::Bar => {
///         type T = Bar;
///         type_name::<T>()
///     }
/// }
/// ```
///
/// Type bindings can also be combined with payload bindings:
///
/// ```
/// use match_variants::{match_variants, MatchVariants};
///
/// struct Foo;
/// struct Bar;
///
/// #[derive(MatchVariants)]
/// enum Value {
///     #[variant_type(Foo)]
///     Foo(i32),
///
///     #[variant_type(Bar)]
///     Bar(i32),
/// }
///
/// let value = Value::Bar(42);
///
/// let result = match_variants!(Value, value, type T, (x), {
///     (std::any::type_name::<T>(), x)
/// });
///
/// assert_eq!(result, (std::any::type_name::<Bar>(), 42));
/// ```
///
/// Requesting a `type` binding for an enum without `#[variant_type(...)]`
/// metadata produces a compile-time error.
///
/// When matching an enum from another module, use [`import_match_variants!`]
/// to bring its generated helper macro into scope.
#[proc_macro]
pub fn match_variants(input: TokenStream) -> TokenStream {
    match_variants::expand(input)
}

/// Imports the helper macros generated by [`MatchVariants`].
///
/// This is needed when enums derived with [`MatchVariants`] are used from
/// another module.
///
/// # Example
///
/// ```
/// mod domain {
///     use match_variants::MatchVariants;
///
///     pub struct Foo(pub i32);
///     pub struct Bar(pub i32);
///
///     impl Foo {
///         pub fn value(self) -> i32 {
///             self.0
///         }
///     }
///
///     impl Bar {
///         pub fn value(self) -> i32 {
///             self.0
///         }
///     }
///
///     #[derive(MatchVariants)]
///     pub enum Value {
///         Foo(Foo),
///         Bar(Bar),
///     }
/// }
///
/// mod application {
///     use match_variants::{import_match_variants, match_variants};
///
///     import_match_variants!(
///         crate::domain::{
///             Value,
///         }
///     );
///
///     pub fn value(value: crate::domain::Value) -> i32 {
///         match_variants!(crate::domain::Value, value, (x), {
///             x.value()
///         })
///     }
/// }
///
/// fn main() {
///     let value = domain::Value::Foo(domain::Foo(42));
///
///     assert_eq!(application::value(value), 42);
/// }
/// ```
///
/// Multiple enums from the same module can be imported at once:
///
/// ```text
/// import_match_variants!(
///     crate::domain::{
///         FooValue,
///         BarValue,
///         BazValue,
///     }
/// );
/// ```
#[proc_macro]
pub fn import_match_variants(input: TokenStream) -> TokenStream {
    import_match_variants::expand(input)
}
