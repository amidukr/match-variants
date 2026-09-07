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

            Err(error) => {
                return error.to_compile_error().into();
            }
        };

        variants.push(VariantInfo {
            ident: variant.ident,
            variant_type,
        });
    }

    let variant_type_count = variants
        .iter()
        .filter(|variant| variant.variant_type.is_some())
        .count();

    if variant_type_count != 0 && variant_type_count != variants.len() {
        let missing_variants: Vec<_> = variants
            .iter()
            .filter(|variant| variant.variant_type.is_none())
            .map(|variant| variant.ident.to_string())
            .collect();

        let first_missing = variants
            .iter()
            .find(|variant| variant.variant_type.is_none())
            .expect("partial variant type metadata must have a missing variant");

        let message = format!(
            "MatchVariants requires #[variant_type(...)] on either all variants or none; \
             missing on: {}",
            missing_variants.join(", "),
        );

        return syn::Error::new_spanned(&first_missing.ident, message)
            .to_compile_error()
            .into();
    }

    let has_variant_types = !variants.is_empty() && variant_type_count == variants.len();

    let helper_macro = format_ident!("match_variants_for_{}", enum_name);

    generate_macro(&helper_macro, &variants, has_variant_types, variant_kind).into()
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

fn generate_macro(
    helper_macro: &Ident,
    variants: &[VariantInfo],
    has_variant_types: bool,
    variant_kind: Option<VariantKind>,
) -> TokenStream2 {
    let generate_rule = |has_pattern: bool, typed: bool| {
        let type_input = if typed {
            quote! {
                [type $type_binding:ident]
            }
        } else {
            quote! {
                [no_type]
            }
        };

        let pattern_input = match (variant_kind, has_pattern) {
            (Some(VariantKind::Unnamed), true) => {
                quote! {
                    [unnamed (
                        $($binding:pat),*
                        $(,)?
                    )]
                }
            }

            (Some(VariantKind::Named), true) => {
                quote! {
                    [named {
                        $(
                            $field:ident
                                :
                            $binding:pat
                        ),*
                        $(,)?
                    }]
                }
            }

            (_, false) => {
                quote! {
                    [none]
                }
            }

            (Some(VariantKind::Unit), true) | (None, true) => {
                quote! {
                    $pattern:tt
                }
            }
        };

        let error = if typed && !has_variant_types {
            Some("`type` binding requires #[variant_type(...)] on every enum variant")
        } else if has_pattern {
            match variant_kind {
                Some(VariantKind::Unit) => Some("cannot use a variant pattern with unit variants"),

                None => Some("cannot use a variant pattern with an empty enum"),

                _ => None,
            }
        } else {
            None
        };

        if let Some(error) = error {
            return quote! {
                (
                    [$first:tt $(:: $rest:tt)*],
                    $value:expr,
                    #type_input,
                    #pattern_input,
                    $body:expr
                ) => {
                    compile_error!(#error)
                };
            };
        }

        let arms = variants.iter().map(|variant| {
            let variant_name = &variant.ident;

            let type_alias = variant
                .variant_type
                .as_ref()
                .filter(|_| typed)
                .map(|variant_type| {
                    quote! {
                        type $type_binding =
                            #variant_type;
                    }
                });

            let variant_pattern = match (variant_kind, has_pattern) {
                (Some(VariantKind::Unit), _) => {
                    quote! {}
                }

                (Some(VariantKind::Unnamed), false) => {
                    quote! {
                        (..)
                    }
                }

                (Some(VariantKind::Unnamed), true) => {
                    quote! {
                        (
                            $($binding),*
                        )
                    }
                }

                (Some(VariantKind::Named), false) => {
                    quote! {
                        { .. }
                    }
                }

                (Some(VariantKind::Named), true) => {
                    quote! {
                        {
                            $(
                                $field:
                                    $binding
                            ),*,
                            ..
                        }
                    }
                }

                (None, _) => {
                    quote! {}
                }
            };

            quote! {
                $first $(:: $rest)*
                    :: #variant_name
                    #variant_pattern
                => {
                    #type_alias
                    $body
                },
            }
        });

        quote! {
            (
                [$first:tt $(:: $rest:tt)*],
                $value:expr,
                #type_input,
                #pattern_input,
                $body:expr
            ) => {
                match $value {
                    #(#arms)*
                }
            };
        }
    };

    let rules = vec![
        generate_rule(false, false),
        generate_rule(false, true),
        generate_rule(true, false),
        generate_rule(true, true),
    ];

    generate_helper_macro(helper_macro, &rules)
}

fn generate_helper_macro(helper_macro: &Ident, rules: &[TokenStream2]) -> TokenStream2 {
    quote! {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! #helper_macro {
            #(#rules)*
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub mod #helper_macro {
            pub use #helper_macro;
        }
    }
}
