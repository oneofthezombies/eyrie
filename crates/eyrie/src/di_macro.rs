use convert_case::Case;
use convert_case::Casing as _;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Ident;
use syn::ItemStruct;

pub(crate) fn injectable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = &item_struct.ident;
    let module_name_str = struct_name.to_string().to_case(Case::Snake);
    let module_name = Ident::new(&module_name_str, struct_name.span());
    let expanded = quote! {
        #item_struct

        mod #module_name {
            use super::*;

            #[ctor::ctor]
            pub(crate) fn register() {
                let mut container = DI_CONTAINER.lock();
                container.register()
            }
        }
    };
    expanded.into()
}

pub(crate) fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    todo!();
}
