use proc_macro::TokenStream;
mod debug_field;

/// Proc macro to remove code when not debug build.
/// ```rust
/// debug! {
///    struct Test {
///      pub i: i32,
///   }
/// }
/// ```
/// In above case, struct `Test` will be removed when not debug build.
#[proc_macro]
pub fn debug(item: TokenStream) -> TokenStream {
    if cfg!(debug_assertions) {
        item
    } else {
        Default::default()
    }
}

// proc_macro_attribute don't support helper attributes
// So use proc_macro instead.
/// Remove the `field` or `variant` with `#[debug]` attribute Struct, Enum or Union.
/// ```rust
/// debug_fields! {
///    struct Test {
///       x: i32,
///       #[debug]
///       y: i32,
///    }
/// }
/// ```
/// In above case, field `y` will be removed when not debug build.
#[proc_macro]
pub fn debug_field(item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::DeriveInput);
    debug_field::debug_field_impl(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
