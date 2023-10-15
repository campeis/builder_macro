extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Builder)]
pub fn derive_builder(item: TokenStream) -> TokenStream {
    builder_code::builder_for(item.into()).into()
}
