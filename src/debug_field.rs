use proc_macro2;
use quote;
use syn::{self, punctuated::Punctuated};

pub fn debug_field_impl(ast: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    match &mut ast.data {
        syn::Data::Struct(ref data) => {
            let new_fields = filter_fields(&data.fields);
            ast.data = syn::Data::Struct(syn::DataStruct {
                fields: new_fields,
                ..data.clone()
            });
            return Ok(quote::quote! { #ast });
        }
        syn::Data::Enum(ref data) => {
            let new_variants = filter_variants(&data.variants);
            ast.data = syn::Data::Enum(syn::DataEnum {
                variants: new_variants,
                ..data.clone()
            });
            return Ok(quote::quote! { #ast });
        }
        syn::Data::Union(ref data) => {
            let new_fileds = filter_punt_fields(&data.fields.named);
            let named_fields = syn::FieldsNamed {
                brace_token: data.fields.brace_token,
                named: new_fileds,
            };
            ast.data = syn::Data::Union(syn::DataUnion {
                fields: named_fields,
                ..data.clone()
            });
            return Ok(quote::quote! { #ast });
        }
    }
}

fn filter_fields(input: &syn::Fields) -> syn::Fields {
    match input {
        syn::Fields::Named(ref fields) => {
            let punts = filter_punt_fields(&fields.named);
            syn::Fields::Named(syn::FieldsNamed {
                brace_token: fields.brace_token,
                named: punts,
            })
        }
        syn::Fields::Unnamed(ref fields) => {
            let punts = filter_punt_fields(&fields.unnamed);
            syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: fields.paren_token,
                unnamed: punts,
            })
        }
        syn::Fields::Unit => syn::Fields::Unit,
    }
}

fn filter_punt_fields(
    input: &Punctuated<syn::Field, syn::Token![,]>,
) -> Punctuated<syn::Field, syn::Token![,]> {
    let mut punts = syn::punctuated::Punctuated::<syn::Field, syn::Token![,]>::new();
    for field in input.iter() {
        if filter_field(field) {
            punts.push(remove_debug_attr_helper(field));
        }
    }
    punts
}

fn remove_debug_attr_helper(field: &syn::Field) -> syn::Field {
    let mut new_attrs = Vec::new();
    for attr in &field.attrs {
        if !attr.path().is_ident("debug") {
            new_attrs.push(attr.clone());
        }
    }
    syn::Field {
        attrs: new_attrs,
        ..field.clone()
    }
}

fn filter_field(field: &syn::Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("debug") {
            if cfg!(debug_assertions) {
                return true;
            } else {
                return false;
            }
        }
    }
    true
}

fn filter_variants(
    input: &Punctuated<syn::Variant, syn::Token![,]>,
) -> Punctuated<syn::Variant, syn::Token![,]> {
    let mut punts = syn::punctuated::Punctuated::<syn::Variant, syn::Token![,]>::new();
    for variant in input.iter() {
        if filter_variant(variant) {
            punts.push(remove_debug_attr_helper_of_variant(variant));
        }
    }
    punts
}

fn remove_debug_attr_helper_of_variant(variant: &syn::Variant) -> syn::Variant {
    let mut new_attrs = Vec::new();
    for attr in &variant.attrs {
        if !attr.path().is_ident("debug") {
            new_attrs.push(attr.clone());
        }
    }
    syn::Variant {
        attrs: new_attrs,
        ..variant.clone()
    }
}

fn filter_variant(variant: &syn::Variant) -> bool {
    for attr in &variant.attrs {
        if attr.path().is_ident("debug") {
            if cfg!(debug_assertions) {
                return true;
            } else {
                return false;
            }
        }
    }
    true
}
