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
    pattern: VariantPattern,
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

        let pattern = if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);

            let bindings =
                Punctuated::<Pat, Token![,]>::parse_terminated_with(&content, Pat::parse_single)?;

            VariantPattern::Unnamed(bindings.into_iter().collect())
        } else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);

            let bindings = Punctuated::<NamedBinding, Token![,]>::parse_terminated(&content)?;

            VariantPattern::Named(bindings.into_iter().collect())
        } else {
            return Err(input.error(
                "expected unnamed pattern `(x, ...)` \
                 or named pattern `{ field: x, ... }`",
            ));
        };

        input.parse::<Token![,]>()?;

        let body: Expr = input.parse()?;

        Ok(Self {
            enum_path,
            value,
            pattern,
            body,
        })
    }
}

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let MatchVariantsInput {
        enum_path,
        value,
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

    let generated = match pattern {
        VariantPattern::Unnamed(bindings) => {
            quote! {{
                #helper_macro!(
                    [#match_enum_path],
                    #value,
                    (#(#bindings),*),
                    #body
                )
            }}
        }

        VariantPattern::Named(bindings) => {
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
