use proc_macro::TokenStream;
mod debug_field;

#[proc_macro]
pub fn debug(item: TokenStream) -> TokenStream {
    if cfg!(debug_assertions) {
        item
    } else {
        return Default::default();
    }
}

// proc_macro_attribute don't support helper attributes
// So use proc_macro instead.
#[proc_macro]
pub fn debug_field(item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::DeriveInput);
    debug_field::debug_field_impl(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
