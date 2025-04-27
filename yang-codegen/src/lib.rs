use proc_macro2::TokenStream;
use quote::quote;
use yang_parser::model::*;

mod utils;

pub fn generate(module: YangModule) {
    if let YangModule::Module(module) = module {
        for node in module.body {
            match node {
                SchemaNode::DataDef(data_def) => match data_def {
                    DataDef::Container(container) => generate_container(container),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

fn generate_container(container: Container) {
    let struct_name = utils::sanitize_identifier(container.name.as_str());
    let struct_fields: Vec<TokenStream> = container
        .data_defs
        .iter()
        .filter_map(|child| match child {
            DataDef::Leaf(leaf) => Some(generate_leaf(leaf)),
            _ => None,
        })
        .collect();
    let doc = match &container.description {
        Some(desc) => format!(" {}", desc.as_str()),
        None => "".into(),
    };

    let struct_def = quote! {
        #[doc = #doc]
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(#struct_fields)*
        }
    };

    let syntax_tree = syn::parse_file(&struct_def.to_string()).expect("Failed to parse generated code");
    let formatted_code = prettyplease::unparse(&syntax_tree);
    println!("{}", formatted_code);
}

fn generate_leaf(leaf: &Leaf) -> TokenStream {
    let field_name = utils::sanitize_identifier(leaf.name.as_str());
    let field_type = utils::yang_to_rust_type(leaf.type_info.name.as_str());
    let doc = match &leaf.description {
        Some(desc) => format!(" {}", desc.as_str()),
        None => "".into(),
    };

    quote! {
        #[doc = #doc]
        pub #field_name: #field_type,
    }
}
