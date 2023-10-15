use builder_macro::Builder;

#[test]
fn can_derive_builder_for_struct_with_no_field() {
    #[derive(Builder)]
    struct StructWithNoField {}

    let _ = StructWithNoFieldBuilder::builder().build();
}

#[test]
fn can_derive_builder_for_struct_with_fields() {
    #[allow(dead_code)]
    #[derive(Builder)]
    struct StructWithField {
        f: i64,
    }

    let _ = StructWithFieldBuilder::builder().build();
}

#[test]
fn builder_has_set_methods() {
    #[allow(dead_code)]
    #[derive(Builder)]
    struct StructWithField {
        f1: i64,
        f2: String,
    }

    let _ = StructWithFieldBuilder::builder()
        .f1(1)
        .f2("string".to_string())
        .build();
}

#[test]
fn built_struct_has_fields_set() {
    #[allow(dead_code)]
    #[derive(Builder)]
    struct StructWithField {
        f1: String,
        f2: String,
    }

    let built = StructWithFieldBuilder::builder()
        .f1("value f1".into())
        .f2("value f2".into())
        .build()
        .unwrap();

    assert_eq!("value f1", built.f1);
    assert_eq!("value f2", built.f2);
}

#[test]
fn build_returns_error_if_a_field_is_not_set() {
    #[allow(dead_code)]
    #[derive(Builder)]
    struct StructWithField {
        field: String,
    }

    let built = StructWithFieldBuilder::builder().build();

    assert!(built.is_err());
}
