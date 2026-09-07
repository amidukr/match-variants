use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Type};

#[derive(Clone, Copy, PartialEq, Eq)]
enum VariantKind {
    Unit,
    Unnamed,
    Named,
}

struct VariantInfo {
    ident: Ident,
    variant_type: Option<Type>,
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
            Fields::Unit => VariantKind::Unit,
            Fields::Unnamed(_) => VariantKind::Unnamed,
            Fields::Named(_) => VariantKind::Named,
        };

        if let Some(expected_kind) = variant_kind {
            if expected_kind != current_kind {
                return syn::Error::new_spanned(
                    variant.ident,
                    "MatchVariants does not support mixing unit, unnamed, and named variants",
                )
                .to_compile_error()
                .into();
            }
        } else {
            variant_kind = Some(current_kind);
        }

        let variant_type = match parse_variant_type(&variant.attrs) {
            Ok(variant_type) => variant_type,
            Err(error) => return error.to_compile_error().into(),
        };

        variants.push(VariantInfo {
            ident: variant.ident,
            variant_type,
        });
    }

    let helper_macro = format_ident!("match_variants_for_{}", enum_name);

    let generated = match variant_kind {
        Some(VariantKind::Unit) => generate_unit_macro(&helper_macro, &variants),

        Some(VariantKind::Unnamed) => generate_unnamed_macro(&helper_macro, &variants),

        Some(VariantKind::Named) => generate_named_macro(&helper_macro, &variants),

        None => generate_empty_macro(&helper_macro),
    };

    generated.into()
}

fn parse_variant_type(attrs: &[Attribute]) -> syn::Result<Option<Type>> {
    let mut variant_type = None;

    for attr in attrs {
        if !attr.path().is_ident("variant_type") {
            continue;
        }

        if variant_type.is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "duplicate #[variant_type(...)] attribute",
            ));
        }

        variant_type = Some(attr.parse_args::<Type>()?);
    }

    Ok(variant_type)
}

fn generate_unit_macro(helper_macro: &Ident, variants: &[VariantInfo]) -> TokenStream2 {
    let variant_names: Vec<_> = variants.iter().map(|variant| &variant.ident).collect();

    let typed_arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.variant_type {
                Some(variant_type) => {
                    quote! {
                        $first $(:: $rest)* :: #variant_name => {
                            type $type_binding = #variant_type;
                            $body
                        },
                    }
                }

                None => {
                    quote! {
                        $first $(:: $rest)* :: #variant_name => {
                            compile_error!(
                                "variant is missing #[variant_type(...)]"
                            );
                        },
                    }
                }
            }
        })
        .collect();

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro {
            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                type $type_binding:ident,
                $body:expr
            ) => {
                match $value {
                    #(#typed_arms)*
                }
            };

            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                $body:expr
            ) => {
                match $value {
                    #(
                        $first $(:: $rest)* :: #variant_names => $body,
                    )*
                }
            };
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub mod #helper_macro {
            pub use #helper_macro;
        }
    }
}

fn generate_unnamed_macro(helper_macro: &Ident, variants: &[VariantInfo]) -> TokenStream2 {
    let variant_names: Vec<_> = variants.iter().map(|variant| &variant.ident).collect();

    let typed_arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.variant_type {
                Some(variant_type) => {
                    quote! {
                        $first $(:: $rest)* :: #variant_name(
                            $($binding),*
                        ) => {
                            type $type_binding = #variant_type;
                            $body
                        },
                    }
                }

                None => {
                    quote! {
                        $first $(:: $rest)* :: #variant_name(
                            $($binding),*
                        ) => {
                            compile_error!(
                                "variant is missing #[variant_type(...)]"
                            );
                        },
                    }
                }
            }
        })
        .collect();

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro {
            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                type $type_binding:ident,
                ($($binding:pat),* $(,)?),
                $body:expr
            ) => {
                match $value {
                    #(#typed_arms)*
                }
            };

            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                ($($binding:pat),* $(,)?),
                $body:expr
            ) => {
                match $value {
                    #(
                        $first $(:: $rest)* :: #variant_names(
                            $($binding),*
                        ) => $body,
                    )*
                }
            };
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub mod #helper_macro {
            pub use #helper_macro;
        }
    }
}

fn generate_named_macro(helper_macro: &Ident, variants: &[VariantInfo]) -> TokenStream2 {
    let variant_names: Vec<_> = variants.iter().map(|variant| &variant.ident).collect();

    let typed_arms: Vec<_> = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.variant_type {
                Some(variant_type) => {
                    quote! {
                        $first $(:: $rest)* :: #variant_name {
                            $(
                                $field: $binding
                            ),*,
                            ..
                        } => {
                            type $type_binding = #variant_type;
                            $body
                        },
                    }
                }

                None => {
                    quote! {
                        $first $(:: $rest)* :: #variant_name {
                            $(
                                $field: $binding
                            ),*,
                            ..
                        } => {
                            compile_error!(
                                "variant is missing #[variant_type(...)]"
                            );
                        },
                    }
                }
            }
        })
        .collect();

    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro {
            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                type $type_binding:ident,
                {
                    $(
                        $field:ident : $binding:pat
                    ),*
                    $(,)?
                },
                $body:expr
            ) => {
                match $value {
                    #(#typed_arms)*
                }
            };

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
                        $first $(:: $rest)* :: #variant_names {
                            $(
                                $field: $binding
                            ),*,
                            ..
                        } => $body,
                    )*
                }
            };
        }

        #[doc(hidden)]
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

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub mod #helper_macro {
            pub use #helper_macro;
        }
    }
}
