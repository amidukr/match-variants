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
        } else if input.peek(syn::token::Brace) && brace_is_followed_by_comma(input)? {
            let pattern = parse_named_pattern(input)?;

            input.parse::<Token![,]>()?;

            Some(pattern)
        } else {
            None
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

    let type_binding = type_binding.map(|type_binding| {
        quote! {
            type #type_binding,
        }
    });

    let pattern = match pattern {
        None => {
            quote! {}
        }

        Some(VariantPattern::Unnamed(bindings)) => {
            quote! {
                (#(#bindings),*),
            }
        }

        Some(VariantPattern::Named(bindings)) => {
            let fields = bindings.iter().map(|binding| &binding.field);

            let patterns = bindings.iter().map(|binding| &binding.binding);

            quote! {
                {
                    #(
                        #fields: #patterns
                    ),*
                },
            }
        }
    };

    quote! {{
        #helper_macro!(
            [#match_enum_path],
            #value,
            #type_binding
            #pattern
            #body
        )
    }}
    .into()
}

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
