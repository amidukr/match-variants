use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, UseTree};

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(input as UseTree);

    match generate_imports(&tree, quote! {}) {
        Ok(generated) => generated.into(),

        Err(error) => error.to_compile_error().into(),
    }
}

fn generate_imports(tree: &UseTree, prefix: TokenStream2) -> syn::Result<TokenStream2> {
    match tree {
        UseTree::Path(path) => {
            let ident = &path.ident;

            let prefix = if prefix.is_empty() {
                quote! {
                    #ident
                }
            } else {
                quote! {
                    #prefix :: #ident
                }
            };

            generate_imports(&path.tree, prefix)
        }

        UseTree::Group(group) => {
            let imports = group
                .items
                .iter()
                .map(|tree| generate_imports(tree, prefix.clone()))
                .collect::<syn::Result<Vec<_>>>()?;

            Ok(quote! {
                #(#imports)*
            })
        }

        UseTree::Name(name) => {
            let enum_name = &name.ident;

            let helper_macro = format_ident!("match_variants_for_{}", enum_name);

            Ok(quote! {
                use #prefix
                    ::#helper_macro
                    ::#helper_macro;
            })
        }

        UseTree::Rename(rename) => Err(syn::Error::new_spanned(
            rename,
            "renaming enums is not supported",
        )),

        UseTree::Glob(glob) => Err(syn::Error::new_spanned(
            glob,
            "glob imports are not supported",
        )),
    }
}
