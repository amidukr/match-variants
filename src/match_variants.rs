use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, Pat, Path, Result, Token,
};

struct MatchVariantsInput {
    enum_path: Path,
    value: Expr,
    type_binding: Option<Ident>,
    pattern: Option<VariantPattern>,
    body: Expr,
}

enum VariantPattern {
    Unnamed(Vec<Pat>),
    Named(Vec<NamedBinding>),
}

struct NamedBinding {
    field: Ident,
    binding: Pat,
}

impl Parse for NamedBinding {
    fn parse(input: ParseStream) -> Result<Self> {
        let field: Ident = input.parse()?;

        input.parse::<Token![:]>()?;

        let binding = Pat::parse_single(input)?;

        Ok(Self { field, binding })
    }
}

impl Parse for MatchVariantsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let enum_path = input.call(Path::parse_mod_style)?;

        input.parse::<Token![,]>()?;

        let value: Expr = input.parse()?;

        input.parse::<Token![,]>()?;

        let type_binding = if input.peek(Token![type]) {
            input.parse::<Token![type]>()?;

            let binding: Ident = input.parse()?;

            input.parse::<Token![,]>()?;

            Some(binding)
        } else {
            None
        };

        let pattern = if input.peek(syn::token::Paren) {
            let pattern = parse_unnamed_pattern(input)?;

            input.parse::<Token![,]>()?;

            Some(pattern)
        } else if input.peek(syn::token::Brace)
            && (type_binding.is_none() || brace_is_followed_by_comma(input)?)
        {
            let pattern = parse_named_pattern(input)?;

            input.parse::<Token![,]>()?;

            Some(pattern)
        } else if type_binding.is_some() {
            None
        } else {
            return Err(input.error(
                "expected unnamed pattern `(x, ...)` \
                 or named pattern `{ field: x, ... }`",
            ));
        };

        let body: Expr = input.parse()?;

        Ok(Self {
            enum_path,
            value,
            type_binding,
            pattern,
            body,
        })
    }
}

fn parse_unnamed_pattern(input: ParseStream) -> Result<VariantPattern> {
    let content;
    parenthesized!(content in input);

    let bindings =
        Punctuated::<Pat, Token![,]>::parse_terminated_with(&content, Pat::parse_single)?;

    Ok(VariantPattern::Unnamed(bindings.into_iter().collect()))
}

fn parse_named_pattern(input: ParseStream) -> Result<VariantPattern> {
    let content;
    braced!(content in input);

    let bindings = Punctuated::<NamedBinding, Token![,]>::parse_terminated(&content)?;

    Ok(VariantPattern::Named(bindings.into_iter().collect()))
}

fn brace_is_followed_by_comma(input: ParseStream) -> Result<bool> {
    let fork = input.fork();

    let content;
    braced!(content in fork);

    Ok(fork.peek(Token![,]))
}

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let MatchVariantsInput {
        enum_path,
        value,
        type_binding,
        pattern,
        body,
    } = parse_macro_input!(input as MatchVariantsInput);

    let enum_name = &enum_path
        .segments
        .last()
        .expect("enum path cannot be empty")
        .ident;

    let helper_macro = format_ident!("match_variants_for_{}", enum_name);

    let match_enum_path = generate_match_enum_path(&enum_path);

    let generated = match (type_binding, pattern) {
        (None, Some(VariantPattern::Unnamed(bindings))) => {
            quote! {{
                #helper_macro!(
                    [#match_enum_path],
                    #value,
                    (#(#bindings),*),
                    #body
                )
            }}
        }

        (None, Some(VariantPattern::Named(bindings))) => {
            let fields = bindings.iter().map(|binding| &binding.field);

            let patterns = bindings.iter().map(|binding| &binding.binding);

            quote! {{
                #helper_macro!(
                    [#match_enum_path],
                    #value,
                    {
                        #(
                            #fields: #patterns
                        ),*
                    },
                    #body
                )
            }}
        }

        (Some(type_binding), None) => {
            quote! {{
                #helper_macro!(
                    [#match_enum_path],
                    #value,
                    type #type_binding,
                    #body
                )
            }}
        }

        (Some(type_binding), Some(VariantPattern::Unnamed(bindings))) => {
            quote! {{
                #helper_macro!(
                    [#match_enum_path],
                    #value,
                    type #type_binding,
                    (#(#bindings),*),
                    #body
                )
            }}
        }

        (Some(type_binding), Some(VariantPattern::Named(bindings))) => {
            let fields = bindings.iter().map(|binding| &binding.field);

            let patterns = bindings.iter().map(|binding| &binding.binding);

            quote! {{
                #helper_macro!(
                    [#match_enum_path],
                    #value,
                    type #type_binding,
                    {
                        #(
                            #fields: #patterns
                        ),*
                    },
                    #body
                )
            }}
        }

        (None, None) => {
            unreachable!("a match without a type binding must have a variant pattern")
        }
    };

    generated.into()
}

/// Returns the enum path used inside the generated match.
///
/// `MyEnum`
///     -> `MyEnum`
///
/// `crate::MyEnum`
///     -> `MyEnum`
///
/// `domain::MyEnum`
///     -> `MyEnum`
///
/// `crate::foo::bar::MyEnum`
///     -> `crate::foo::bar::MyEnum`
///
/// `domain::foo::bar::MyEnum`
///     -> `domain::foo::bar::MyEnum`
///
/// A two-segment path is treated as
/// `<helper crate/module>::<locally imported enum>`.
///
/// Paths with three or more segments are treated as full enum paths.
fn generate_match_enum_path(enum_path: &Path) -> TokenStream2 {
    if enum_path.segments.len() <= 2 {
        let enum_name = &enum_path
            .segments
            .last()
            .expect("enum path cannot be empty")
            .ident;

        quote! {
            #enum_name
        }
    } else {
        quote! {
            #enum_path
        }
    }
}
