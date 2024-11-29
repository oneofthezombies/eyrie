use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

pub fn router_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(args as LitStr).value();
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();
    let expanded = quote! {
        #input

        impl #struct_name {
            pub fn make_router(state: i32) -> axum::Router {
                axum::Router::new()
            }
        }
    };
    expanded.into()
}
