use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use roxmltree as rx;

fn id(str: &str) -> Ident {
    quote::format_ident!("{}", str)
}

fn convert_field_name(str: &str) -> String {
    match str {
        "type" => String::from("type_"),
        "static" => String::from("static_"),
        "ref" => String::from("ref_"),
        "const" => String::from("const_"),
        "abstract" => String::from("abstract_"),
        "final" => String::from("final_"),
        _ => String::from(str),
    }
}

fn convert_field_type(str: &str) -> String {
    match str {
        "xsd:string" => String::from("String"),
        "xsd:integer" => String::from("i32"),
        "DoxBool" => String::from("bool"),
        "DoxOlType" => String::from("String"),
        _ => String::from(str.to_upper_camel_case()),
    }
}

fn is_mixed_content(element: &rx::Node) -> bool {
    // if element.attrib["name"] in mixed_exceptions:
    //     return False
    element.attribute("mixed").is_some()
}

fn is_simple_content(element: &rx::Node) -> bool {
    false
}

fn create_struct(element: rx::Node) -> anyhow::Result<TokenStream> {
    let mut attributes = Vec::new();
    for child in element.children() {
        match child.tag_name().name() {
            "attribute" => match (child.attribute("name"), child.attribute("type")) {
                (Some(field_name), Some(field_type)) => {
                    let field_name = id(&convert_field_name(field_name));
                    let field_type = id(&convert_field_type(field_type));
                    attributes.push(quote! { #field_name: #field_type });
                }
                _ => {}
            },
            name => {
                println!("{name}");
            }
        }
    }

    let name = element
        .attribute("name")
        .context("Failed to get name attribute")?
        .to_upper_camel_case();

    let name = id(&name);

    Ok(quote! {
        struct #name {
            #(#attributes),*
        }
    })
}

fn create_mixed_content(element: rx::Node) -> anyhow::Result<TokenStream> {
    Ok(quote! { struct MixedContent; })
}

fn create_simple_content(element: rx::Node) -> anyhow::Result<TokenStream> {
    Ok(quote! { struct SimpleContent; })
}

fn generate_mod_from_xsd(mod_name: &str, xsd_path: &Path) -> anyhow::Result<()> {
    let xml_str = std::fs::read_to_string(&xsd_path)?;
    let doc = rx::Document::parse(&xml_str)?;

    let schema = doc
        .root()
        .first_element_child()
        .context("Failed to get first element")?;

    let mut nodes = Vec::new();

    for child in schema.children() {
        // println!("{child:?}");
        // println!("{:?}", child.tag_name().name());

        match child.tag_name().name() {
            "complexType" => {
                let ast = if is_simple_content(&child) {
                    create_simple_content(child)?
                } else if is_mixed_content(&child) {
                    create_mixed_content(child)?
                } else {
                    create_struct(child)?
                };

                nodes.push(ast);
            }
            "simpleType" => {}
            "group" => {}
            _ => {}
        }
    }

    let file_ast = quote! {
        #[allow(dead_code)]
        #(#nodes)*
    };

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let dir = out_dir.join("xsds");

    std::fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{mod_name}.rs"));

    // eprintln!("ast {}", ast.to_token_stream().to_string());

    std::fs::write(&path, file_ast.to_token_stream().to_string())?;

    let output = std::process::Command::new("rustfmt").arg(&path).output()?;
    if !output.status.success() {
        anyhow::bail!("Failed to run rustfmt on {}", path.display());
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    generate_mod_from_xsd("compound", &PathBuf::from("xsd/compound.xsd"))?;

    Ok(())
}
