use crate::builder_for;
use quote::quote;

#[test]
fn builder_name_starts_with_struct_name() {
    let input = quote! {
        struct EmptyStruct {}
    };

    let actual = builder_for(input);

    assert!(actual.to_string().contains("struct EmptyStructBuilder"));
}

#[test]
fn builder_has_setter_for_a_field() {
    let input = quote! {
        struct Struct {
            field_name: i64,
            field_name2: String,
        }
    };

    let actual = builder_for(input);

    assert!(actual.to_string().contains("fn field_name"));
}
