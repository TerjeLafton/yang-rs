use proc_macro2::TokenStream;
use quote::format_ident;

pub fn yang_to_rust_type(yang_type: &str) -> proc_macro2::TokenStream {
    let rust_type = match yang_type.trim().to_lowercase().as_str() {
        "int8" => "i8",
        "int16" => "i16",
        "int32" => "i32",
        "int64" => "i64",
        "uint8" => "u8",
        "uint16" => "u16",
        "uint32" => "u32",
        "uint64" => "u64",
        "decimal64" => "f64",
        "string" => "String",
        "boolean" => "bool",
        "empty" => "()",
        _ => panic!("Unknown YANG type: {}", yang_type),
    };

    rust_type.parse::<TokenStream>().expect("Failed to parse Rust type")
}

pub fn sanitize_identifier(id: &str) -> syn::Ident {
    let sanitized = id.replace("-", "_");
    format_ident!("{}", sanitized)
}

pub fn format_docstring(input: &Option<String>) -> String {
    match input {
        Some(doc) => format!(" {}", doc),
        None => "".into(),
    }
}
