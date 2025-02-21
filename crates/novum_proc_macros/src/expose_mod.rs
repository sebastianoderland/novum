use proc_macro::TokenStream;
use quote::quote;

pub fn run(item: TokenStream) -> TokenStream {
    let ExposeMod { mod_name, features } = syn::parse_macro_input!(item as ExposeMod);
    if features.is_empty() {
        return quote! {
            pub use super::#mod_name;
            pub use super::#mod_name::prelude::*;
        }
        .into();
    } else {
        return quote! {
            #[cfg(all( #(feature = #features),* ))]
            pub use super::#mod_name;

            #[cfg(all( #(feature = #features),* ))]
            pub use super::#mod_name::prelude::*;
        }
        .into();
    }
}

pub struct ExposeMod {
    mod_name: syn::Ident,
    features: Vec<syn::LitStr>,
}

impl syn::parse::Parse for ExposeMod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mod_name = input.parse()?;
        let mut features = Vec::new();
        if input.peek(syn::Token![,]) {
            // consume the comma
            let _comma: syn::Token![,] = input.parse()?;
            // check if there is a bracketed list
            if input.peek(syn::token::Bracket) {
                let content;
                syn::bracketed!(content in input);
                while !content.is_empty() {
                    let feature_str: syn::LitStr = content.parse()?;
                    features.push(feature_str);

                    // If there is another comma inside the brackets, consume it
                    if content.peek(syn::Token![,]) {
                        let _comma: syn::Token![,] = content.parse()?;
                    }
                }
            }
        }

        Ok(Self { mod_name, features })
    }
}
