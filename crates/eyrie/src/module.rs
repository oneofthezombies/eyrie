use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub(crate) fn module_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let module_name = input.ident;
    let expanded = quote! {
        impl #module_name {
            pub fn build_router() -> axum::Router {
                let module = #module_name::builder().build();
                let mut router = axum::Router::new();
                for route in module.routes() {
                    router = router.merge(route);
                }
                router
            }
        }
    };
    expanded.into()
}
