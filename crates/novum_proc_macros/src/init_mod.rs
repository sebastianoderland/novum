use proc_macro::TokenStream;
use quote::quote;

/// Example usage:
/// ```
/// init_mod!{
///     files => [
///         logger,
///         sink,
///     ],
///     modules => [
///         sinks,
///     ],
///     external => [
///         extern_time,
///     ],
/// };
/// ```
///
pub fn run(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as InitMod);

    let files = input.files.iter().map(|f| {
        quote! {

        }
    });

    let modules = input.modules.iter().map(|m| {
        quote! {
            pub mod #m;
        }
    });

    let external = input.external.iter().map(|e| {
        quote! {
            pub use #e::*;
        }
    });

    let modules =

    let output = quote! {
        #(#files)*
        #(#modules)*
        #(#external)*
        pub(crate) mod prelude {
            #(#files)*
        }
    };

    output.into()
}

pub struct InitMod {
    files: Vec<syn::Ident>,
    modules: Vec<syn::Ident>,
    external: Vec<syn::Ident>,
}

fn consume_comma(input: syn::parse::ParseStream) -> syn::Result<()> {
    if input.peek(syn::Token![,]) {
        input.parse::<syn::Token![,]>().map(|_| ())
    } else {
        Ok(())
    }
}

impl syn::parse::Parse for InitMod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut files = Vec::new();
        let mut modules = Vec::new();
        let mut external = Vec::new();

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().as_str() {
                "files" => {
                    input.parse::<syn::Token![=>]>()?;
                    if input.peek(syn::token::Bracket) {
                        let content;
                        syn::bracketed!(content in input);
                        while !content.is_empty() {
                            let feature_str: syn::Ident = content.parse()?;
                            files.push(feature_str);
                            consume_comma(&content)?;
                        }
                    }
                    input.parse::<syn::Token![,]>()?;
                }
                "modules" => {
                    input.parse::<syn::Token![=>]>()?;
                    if input.peek(syn::token::Bracket) {
                        let content;
                        syn::bracketed!(content in input);
                        while !content.is_empty() {
                            let feature_str: syn::Ident = content.parse()?;
                            modules.push(feature_str);
                            consume_comma(&content)?;
                        }
                    }
                    input.parse::<syn::Token![,]>()?;
                }
                "external" => {
                    input.parse::<syn::Token![=>]>()?;
                    if input.peek(syn::token::Bracket) {
                        let content;
                        syn::bracketed!(content in input);
                        while !content.is_empty() {
                            let feature_str: syn::Ident = content.parse()?;
                            external.push(feature_str);
                            consume_comma(&content)?;
                        }
                    }
                    input.parse::<syn::Token![,]>()?;
                }
                _ => return Err(syn::Error::new(ident.span(), "unexpected token")),
            }
        }

        Ok(Self {
            files,
            modules,
            external,
        })
    }
}
