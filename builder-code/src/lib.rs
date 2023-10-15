use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter::Map;
use syn::punctuated::{Iter, Punctuated};
use syn::token::Comma;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse2, DataStruct, DeriveInput, Field, FieldsNamed};

#[cfg(test)]
mod tests;

pub fn builder_for(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(item).unwrap();
    let name = ast.ident;
    let builder_ident = format_ident!("{}Builder", name);

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only implemented for structs"),
    };

    let fields_declarations = fields_declarations(fields);
    let fields_initializations = fields_initializations(fields);
    let fields_setters = fields_setters(fields);
    let fields_initializations_in_result_struct = fields_initializations_in_result_struct(fields);

    quote! {
        struct #builder_ident {
            #(#fields_declarations,)*
        }
        impl #builder_ident {
            pub fn builder() -> #builder_ident {
                Self {
                    #(#fields_initializations,)*
                }
            }

            pub fn build(self) -> Result<#name, String> {
                Ok(#name {
                    #(#fields_initializations_in_result_struct,)*
                })
            }

            #(#fields_setters)*
        }
    }
}

fn fields_initializations_in_result_struct(
    fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> TokenStream> {
    fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: self.#field_name.ok_or(format!("field #field_name was not set"))?
        }
    })
}

fn fields_setters(
    fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> TokenStream> {
    fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub fn #field_name(mut self, v: #field_type) -> Self {
                self.#field_name = Some(v);
                self
            }
        }
    })
}

fn fields_initializations(
    fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> TokenStream> {
    fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: None
        }
    })
}

fn fields_declarations(
    fields: &Punctuated<Field, Comma>,
) -> Map<Iter<Field>, fn(&Field) -> TokenStream> {
    fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #field_name: Option<#field_type>
        }
    })
}
