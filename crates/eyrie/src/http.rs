use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct, LitStr};

pub(crate) fn router(args: TokenStream, input: TokenStream) -> TokenStream {
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

pub(crate) fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn connect(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn options(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn trace(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}

pub(crate) fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!();
}
