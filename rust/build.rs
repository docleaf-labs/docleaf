use std::collections::HashSet;
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

fn convert_type(str: &str) -> String {
    match str {
        "xsd:string" => String::from("String"),
        "xsd:integer" => String::from("i32"),
        "DoxBool" => String::from("bool"),
        "DoxOlType" => String::from("String"),
        _ => String::from(str.to_upper_camel_case()),
    }
}

fn convert_enum_name(name: &str) -> String {
    let name = name.replace("#", "Sharp");
    let name = name.replace("+", "Plus");
    name.to_upper_camel_case()
}

fn is_mixed_content(element: &rx::Node) -> bool {
    let mixed_exceptions = vec![Some("enumvalueType")];
    if mixed_exceptions.contains(&element.attribute("name")) {
        false
    } else {
        element.attribute("mixed").is_some()
    }
}

fn is_simple_content(element: &rx::Node) -> bool {
    element
        .children()
        .find(|child| child.tag_name().name() == "simpleContent")
        .is_some()
}

fn create_struct(element: rx::Node) -> anyhow::Result<TokenStream> {
    let name = id(&convert_type(
        element
            .attribute("name")
            .context("Failed to get name attribute")?,
    ));

    // Skip DocEmptyType as it represents nothing
    if name == "DocEmptyType" {
        return Ok(TokenStream::new());
    }

    let attributes = get_attribute_fields(&element);

    let element_fields = get_element_fields(&element)?;

    Ok(quote! {
        struct #name {
            #attributes
            #element_fields
        }
    })
}

fn create_mixed_content(element: rx::Node) -> anyhow::Result<TokenStream> {
    let name = convert_type(
        element
            .attribute("name")
            .context("Failed to get name attribute")?,
    );

    let name_id = id(&name);

    let attributes = get_attribute_fields(&element);

    let mut entries = Vec::new();

    for child in element.children() {
        match child.tag_name().name() {
            "sequence" | "choice" => {
                let mut enum_entries = child
                    .children()
                    .flat_map(
                        |child| match (child.attribute("name"), child.attribute("type")) {
                            (Some(name), Some(type_)) => {
                                let name = id(&convert_field_name(name));
                                let type_ = id(&convert_type(type_));

                                Some(quote! {
                                    #name(#type_),
                                })
                            }
                            _ => None,
                        },
                    )
                    .collect::<Vec<_>>();

                enum_entries.push(quote! { Text(String) });
                entries.append(&mut enum_entries)
            }
            "group" => {
                if let Some(ref_) = child.attribute("ref") {
                    let type_name = convert_type(ref_);
                    entries.push(quote! {
                        #type_name(#type_name),
                        Text(String),
                    })
                }
            }
            _ => {}
        }
    }

    if entries.is_empty() {
        Ok(quote! {
            struct #name_id {
                #attributes
                pub content: String,
            }
        })
    } else {
        let item_id = id(&format!("{name}Item"));
        Ok(quote! {
            struct #name_id {
                #attributes
                pub content: Vec<#item_id>,
            }

            enum #item_id {
            }
        })
    }
}

fn create_simple_content(element: rx::Node) -> anyhow::Result<TokenStream> {
    let name = id(&convert_type(
        element
            .attribute("name")
            .context("Unable to get name attribute")?,
    ));

    let Some(simple_content) = element
        .children()
        .find(|child| child.tag_name().name() == "simpleContent") else {
        return Ok(TokenStream::new())
    };

    let Some(extension) = simple_content.children().find(|child| child.tag_name().name() == "extension") else {
        return Ok(TokenStream::new())
    };

    let content_type = extension
        .attribute("base")
        .context("Unable to find base attribute")?;

    let content_type = id(&convert_type(content_type));

    let attributes = get_attribute_fields(&extension);

    Ok(quote! {
        struct #name {
            #attributes
            pub content: #content_type,
        }
    })
}

enum Wrapper {
    Vec,
    Vec1,
    Option,
}

fn get_wrapper(element: &rx::Node) -> anyhow::Result<Option<Wrapper>> {
    match (
        element.attribute("minOccurs"),
        element.attribute("maxOccurs"),
    ) {
        (Some("0"), Some("1") | None) => Ok(Some(Wrapper::Option)),
        (Some("0"), Some("unbounded")) => Ok(Some(Wrapper::Vec)),
        (Some("1") | None, Some("unbounded")) => Ok(Some(Wrapper::Vec1)),
        (Some("1") | None, Some("1") | None) => Ok(None),
        (min, max) => anyhow::bail!("No wrapper found for {min:?} {max:?}"),
    }
}

fn get_element_fields(element: &rx::Node) -> anyhow::Result<TokenStream> {
    let mut stream = TokenStream::new();

    for child in element.children() {
        match child.tag_name().name() {
            "sequence" => stream.extend(get_element_fields(&child)?),
            "choice" => stream.extend(get_element_fields(&child)?),
            "element" => {
                if let Some(name) = child.attribute("name") {
                    let name = id(&convert_field_name(name));
                    let type_ = id(&convert_type(child.attribute("type").unwrap_or("String")));
                    match get_wrapper(&child)? {
                        Some(Wrapper::Vec) => stream.extend(quote! { #name: Vec<#type_>, }),
                        Some(Wrapper::Vec1) => stream.extend(quote! { #name: vec1::Vec1<#type_>, }),
                        Some(Wrapper::Option) => stream.extend(quote! { #name: Option<#type_>, }),
                        None => stream.extend(quote! { #name: #type_, }),
                    }
                }
            }
            _ => {}
        }
    }

    Ok(stream)
}

fn get_attribute_fields(element: &rx::Node) -> TokenStream {
    let entries = element
        .children()
        .filter(|child| child.tag_name().name() == "attribute")
        .flat_map(
            |child| match (child.attribute("name"), child.attribute("type")) {
                (Some(name), Some(type_)) => {
                    let name = id(&convert_field_name(name));
                    let type_ = id(&convert_type(type_));

                    if child.attribute("use") == Some("optional") {
                        Some(quote! {
                            #name: #type_,
                        })
                    } else {
                        Some(quote! {
                            #name: Option<#type_>,
                        })
                    }
                }
                _ => None,
            },
        )
        .collect::<Vec<_>>();

    quote! {
        #(#entries)*
    }
}

fn create_restriction(name: &str, element: rx::Node) -> anyhow::Result<TokenStream> {
    let name_id = id(&convert_type(name));

    if name == "DoxVersionNumber" || name == "DoxCharRange" {
        return Ok(quote! { type #name_id = String; });
    }

    if name == "DoxOlType" || name == "DoxBool" {
        return Ok(TokenStream::new());
    }

    let mut entries = Vec::new();
    for child in element.children() {
        if child.tag_name().name() == "enumeration" {
            let entry_name = child.attribute("value").context("Failed to get value")?;
            let entry_name = convert_enum_name(entry_name);

            entries.push(id(&entry_name));
        }
    }

    Ok(quote! {
        enum #name_id {
            #(#entries),*
        }
    })
}

fn handle_complex_type(element: rx::Node) -> anyhow::Result<TokenStream> {
    if is_simple_content(&element) {
        create_simple_content(element)
    } else if is_mixed_content(&element) {
        create_mixed_content(element)
    } else {
        create_struct(element)
    }
}

fn handle_simple_type(element: rx::Node) -> anyhow::Result<TokenStream> {
    let mut stream = TokenStream::new();

    let name = element
        .attribute("name")
        .context("Failed to get name for simple type")?;

    println!("simple type {}", name);

    for child in element.children() {
        match child.tag_name().name() {
            "restriction" => stream.extend(create_restriction(name, child)),
            _ => {}
        }
    }

    Ok(stream)
}

fn get_choice_entries(element: rx::Node) -> anyhow::Result<TokenStream> {
    let mut stream = TokenStream::new();

    let mut already_seen = HashSet::new();

    for child in element.children() {
        match child.tag_name().name() {
            "group" => {
                let ref_ = child
                    .attribute("ref")
                    .context("Failed to get ref attribute")?;
                let type_name = id(&convert_type(ref_));
                stream.extend(quote! {
                    #type_name(#type_name),
                })
            }
            "element" => match (child.attribute("name"), child.attribute("type")) {
                (Some(name), Some(type_)) => {
                    let name = convert_type(name);
                    let name_id = id(&name);
                    let type_ = id(&convert_type(type_));

                    if already_seen.insert(name) {
                        if type_ == "DocEmptyType" {
                            stream.extend(quote! {
                                #name_id,
                            })
                        } else {
                            stream.extend(quote! {
                                #name_id(#type_),
                            })
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(stream)
}

fn handle_group(element: rx::Node) -> anyhow::Result<TokenStream> {
    let name = element
        .attribute("name")
        .context("Failed to get name attribute in restriction")?;

    let name_id = id(&convert_type(name));

    let mut enum_entries = TokenStream::new();

    for child in element.children() {
        if child.tag_name().name() == "choice" {
            enum_entries.extend(get_choice_entries(child)?);
        }
    }

    Ok(quote! {
        enum #name_id {
            #enum_entries
        }
    })
}

fn generate_mod_from_xsd(mod_name: &str, xsd_path: &Path) -> anyhow::Result<()> {
    let xml_str = std::fs::read_to_string(&xsd_path)?;
    let doc = rx::Document::parse(&xml_str)?;

    let schema = doc
        .root()
        .first_element_child()
        .context("Failed to get first element")?;

    let mut nodes = TokenStream::new();

    for child in schema.children() {
        match child.tag_name().name() {
            "complexType" => nodes.extend(handle_complex_type(child)?),
            "simpleType" => nodes.extend(handle_simple_type(child)?),
            "group" => nodes.extend(handle_group(child)?),
            _ => {}
        }
    }

    let file_ast = quote! {
        #[allow(dead_code)]
        #nodes
    };

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let dir = out_dir.join("xsds");

    std::fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{mod_name}.rs"));

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
