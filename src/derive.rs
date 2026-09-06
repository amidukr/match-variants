use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[derive(Clone, Copy, PartialEq, Eq)]
enum VariantKind {
    Unnamed,
    Named,
}

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;

    let Data::Enum(data_enum) = input.data else {
        return syn::Error::new_spanned(enum_name, "MatchVariants can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut variants = Vec::new();
    let mut variant_kind = None;

    for variant in data_enum.variants {
        let current_kind = match &variant.fields {
            Fields::Unnamed(_) => VariantKind::Unnamed,
            Fields::Named(_) => VariantKind::Named,

            Fields::Unit => {
                return syn::Error::new_spanned(
                    variant.ident,
                    "MatchVariants does not support unit variants",
                )
                .to_compile_error()
                .into();
            }
        };

        if let Some(expected_kind) = variant_kind {
            if expected_kind != current_kind {
                return syn::Error::new_spanned(
                    variant.ident,
                    "MatchVariants does not support mixing unnamed and named variants",
                )
                .to_compile_error()
                .into();
            }
        } else {
            variant_kind = Some(current_kind);
        }

        variants.push(variant.ident);
    }

    let helper_macro = format_ident!("match_variants_for_{}", enum_name);

    let generated = match variant_kind {
        Some(VariantKind::Unnamed) => generate_unnamed_macro(&helper_macro, &variants),

        Some(VariantKind::Named) => generate_named_macro(&helper_macro, &variants),

        None => generate_empty_macro(&helper_macro),
    };

    generated.into()
}

fn generate_unnamed_macro(helper_macro: &Ident, variants: &[Ident]) -> TokenStream2 {
    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro {
            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                ($($binding:pat),* $(,)?),
                $body:expr
            ) => {
                match $value {
                    #(
                        $first $(:: $rest)* :: #variants(
                            $($binding),*
                        ) => $body,
                    )*
                }
            };
        }

        #[allow(non_snake_case)]
        pub mod #helper_macro {
            pub use #helper_macro;
        }
    }
}

fn generate_named_macro(helper_macro: &Ident, variants: &[Ident]) -> TokenStream2 {
    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro {
            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                {
                    $(
                        $field:ident : $binding:pat
                    ),*
                    $(,)?
                },
                $body:expr
            ) => {
                match $value {
                    #(
                        $first $(:: $rest)* :: #variants {
                            $(
                                $field: $binding
                            ),*,
                            ..
                        } => $body,
                    )*
                }
            };
        }

        #[allow(non_snake_case)]
        pub mod #helper_macro {
            pub use #helper_macro;
        }
    }
}

fn generate_empty_macro(helper_macro: &Ident) -> TokenStream2 {
    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro {
            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                $pattern:tt,
                $body:expr
            ) => {
                match $value {}
            };
        }

        #[allow(non_snake_case)]
        pub mod #helper_macro {
            pub use #helper_macro;
        }
    }
}
