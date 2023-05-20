use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Context as AnyhowContext;
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use roxmltree as rx;

fn id(str: &str) -> Ident {
    format_ident!("{}", str)
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

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Integer,
    String,
    Enum(String),
}

impl Type {
    fn from_str(str: &str) -> Self {
        match str {
            "xsd:integer" => Self::Integer,
            "xsd:string" => Self::String,
            _ => Self::Enum(str.to_string()),
        }
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            Self::Integer => quote! { i32 },
            Self::String => quote! { String },
            Self::Enum(name) => {
                let name = id(&name);
                quote! { #name }
            }
        }
    }

    fn to_type_id(&self) -> TokenStream {
        match self {
            Self::Integer => quote! { i32 },
            Self::String => quote! { String },
            Self::Enum(name) => {
                let name = id(&name.to_upper_camel_case());
                quote! { #name }
            }
        }
    }

    fn to_type_str(&self) -> String {
        match self {
            Self::Integer => "i32".to_string(),
            Self::String => "String".to_string(),
            Self::Enum(name) => name.to_upper_camel_case(),
        }
    }

    fn to_parse_call(&self) -> TokenStream {
        match self {
            Self::Integer => quote! {
                xml::parse_text(reader)?.parse::<i32>()?
            },
            Self::String => quote! {
                xml::parse_text(reader)?
            },
            Self::Enum(name) => {
                let name = id(&name.to_upper_camel_case());
                quote! { #name::parse(reader, tag)? }
            }
        }
    }

    fn to_parse_empty_call(&self) -> Option<TokenStream> {
        match self {
            Self::Integer => None,
            Self::String => Some(quote! {
                String::new()
            }),
            Self::Enum(name) => {
                let name = id(&name.to_upper_camel_case());
                Some(quote! { #name::parse_empty(tag)? })
            }
        }
    }
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

struct Element {
    name: String,
    safe_name: String,
    type_: Type,
    wrapper: Option<Wrapper>,
}

impl Element {
    fn to_field(&self) -> TokenStream {
        let name = id(&self.safe_name.clone());
        let type_ = self.type_.to_type_id();

        match self.wrapper {
            Some(Wrapper::Vec) => {
                quote! { pub #name: Vec<#type_> }
            }
            Some(Wrapper::Vec1) => {
                quote! { pub #name: vec1::Vec1<#type_> }
            }
            Some(Wrapper::Option) => {
                quote! { pub #name: Option<#type_> }
            }
            None => {
                quote! { pub #name: #type_ }
            }
        }
    }

    fn to_mut_init(&self) -> TokenStream {
        let name = id(&self.safe_name.clone());
        match self.wrapper {
            Some(Wrapper::Vec) => {
                quote! { let mut #name = Vec::new() }
            }
            Some(Wrapper::Vec1) => {
                // Ordinary Vec but we'll check for it being non-empty before when parsing the
                // xml element
                quote! { let mut #name = Vec::new() }
            }
            Some(Wrapper::Option) => {
                quote! { let mut #name = None }
            }
            None => {
                // None but we'll check for a value before when parsing the
                // xml element
                quote! { let mut #name = None }
            }
        }
    }

    fn to_init(&self) -> TokenStream {
        let name = id(&self.safe_name.clone());
        match self.wrapper {
            Some(Wrapper::Vec) => {
                quote! { let #name = Vec::new() }
            }
            Some(Wrapper::Vec1) => {
                // Ordinary Vec but we'll check for it being non-empty before when parsing the
                // xml element
                quote! { let #name = Vec::new() }
            }
            Some(Wrapper::Option) => {
                quote! { let #name = None }
            }
            None => {
                // None but we'll check for a value before when parsing the
                // xml element
                quote! { let #name = None }
            }
        }
    }

    fn to_unpack(&self) -> Option<TokenStream> {
        let name = id(&self.safe_name.clone());

        match self.wrapper {
            Some(Wrapper::Vec) => None,
            Some(Wrapper::Vec1) => Some(
                quote! { let #name = vec1::Vec1::try_from_vec(#name).context("Vec was empty")?; },
            ),
            Some(Wrapper::Option) => None,
            None => {
                let name_str = self.safe_name.clone();
                Some(
                    quote! { let #name = #name.with_context(|| format!("Failed to find value for {}", #name_str))?; },
                )
            }
        }
    }

    fn to_match(&self) -> TokenStream {
        let name_string = proc_macro2::Literal::byte_string(self.name.as_bytes());
        let name_var = id(&self.safe_name);

        let parse_call = self.type_.to_parse_call();

        match self.wrapper {
            Some(Wrapper::Vec) => quote! {
                #name_string => {
                    #name_var.push(#parse_call);
                }
            },
            Some(Wrapper::Vec1) => quote! {
                #name_string => {
                    #name_var.push(#parse_call);
                }
            },
            Some(Wrapper::Option) => quote! {
                #name_string => {
                    #name_var = Some(#parse_call);
                }
            },
            None => quote! {
                #name_string => {
                    #name_var = Some(#parse_call);
                }
            },
        }
    }

    fn to_empty_match(&self) -> TokenStream {
        let name_string = proc_macro2::Literal::byte_string(self.name.as_bytes());
        let name_var = id(&self.safe_name);

        let parse_call = self.type_.to_parse_empty_call();

        match self.wrapper {
            Some(Wrapper::Vec) => quote! {
                #name_string => {
                    #name_var.push(#parse_call);
                }
            },
            Some(Wrapper::Vec1) => quote! {
                #name_string => {
                    #name_var.push(#parse_call);
                }
            },
            Some(Wrapper::Option) => quote! {
                #name_string => {
                    #name_var = Some(#parse_call);
                }
            },
            None => quote! {
                #name_string => {
                    #name_var = Some(#parse_call);
                }
            },
        }
    }
}

trait ElementTokenStream {
    fn to_names_stream(&self) -> TokenStream;
    fn to_fields_stream(&self) -> TokenStream;
    fn to_mut_init_stream(&self) -> TokenStream;
    fn to_init_stream(&self) -> TokenStream;
    fn to_unpack_stream(&self) -> TokenStream;
    fn to_matches_stream(&self) -> TokenStream;
    fn to_empty_matches_stream(&self) -> TokenStream;
}

impl ElementTokenStream for Vec<Element> {
    fn to_names_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(|element| id(&element.safe_name));
            // Include trailing comma here as we know we have fields
            quote! { #(#entries),*, }
        }
    }

    fn to_fields_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(Element::to_field);
            // Include trailing comma here as we know we have fields
            quote! { #(#entries),*, }
        }
    }

    fn to_mut_init_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(Element::to_mut_init);
            // Include trailing semi-colon here as we know we have fields
            quote! { #(#entries);*; }
        }
    }

    fn to_init_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(Element::to_init);
            // Include trailing semi-colon here as we know we have fields
            quote! { #(#entries);*; }
        }
    }

    fn to_unpack_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().flat_map(Element::to_unpack);
            // Include trailing semi-colon here as we know we have fields
            quote! { #(#entries);*; }
        }
    }

    fn to_matches_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().flat_map(Element::to_match);
            // Include trailing semi-colon here as we know we have fields
            quote! { #(#entries)* }
        }
    }

    fn to_empty_matches_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().flat_map(Element::to_empty_match);
            // Include trailing semi-colon here as we know we have fields
            quote! { #(#entries)* }
        }
    }
}

struct Attribute {
    name: String,
    safe_name: String,
    type_: Type,
    optional: bool,
}

impl Attribute {
    fn to_field(&self) -> TokenStream {
        let name = id(&self.safe_name);
        let type_ = self.type_.to_token_stream();

        if self.optional {
            quote! { pub #name: Option<#type_> }
        } else {
            quote! { pub #name: #type_ }
        }
    }

    fn to_init(&self) -> TokenStream {
        let field_name = id(&self.safe_name);
        let attr_name = proc_macro2::Literal::byte_string(self.name.as_bytes());
        // TODO: Move these extra parse code into xml module helpers rather than having it inline here
        match self.type_ {
            Type::Integer => {
                if self.optional {
                    quote! {
                        let #field_name = xml::get_optional_attribute_string(#attr_name, &start_tag)?
                                                .map(|str| str.parse::<i32>()).transpose()?
                    }
                } else {
                    quote! {
                        let #field_name = xml::get_attribute_string(#attr_name, &start_tag)?.parse::<i32>()?
                    }
                }
            }
            Type::String => {
                if self.optional {
                    quote! {
                        let #field_name = xml::get_optional_attribute_string(#attr_name, &start_tag)?
                    }
                } else {
                    quote! {
                        let #field_name = xml::get_attribute_string(#attr_name, &start_tag)?
                    }
                }
            }
            Type::Enum(ref enum_name) => {
                let enum_name_id = id(enum_name);
                if self.optional {
                    quote! {
                        let #field_name = xml::get_optional_attribute_enum::<#enum_name_id>(#attr_name, &start_tag)?
                    }
                } else {
                    quote! {
                        let #field_name = xml::get_attribute_enum::<#enum_name_id>(#attr_name, &start_tag)?
                    }
                }
            }
        }
    }

    fn to_unpack(&self) -> Option<TokenStream> {
        let name = id(&self.safe_name);

        if self.optional {
            None
        } else {
            Some(quote! { let #name = #name.context("Failed to find value")?; })
        }
    }
}

trait AttributeTokenStream {
    fn to_names_stream(&self) -> TokenStream;
    fn to_fields_stream(&self) -> TokenStream;
    fn to_init_stream(&self) -> TokenStream;
    fn to_unpack_stream(&self) -> TokenStream;
}

impl AttributeTokenStream for Vec<Attribute> {
    fn to_names_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(|attr| id(&attr.safe_name));
            // Include trailing comma here as we know we have fields
            quote! { #(#entries),*, }
        }
    }

    fn to_fields_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(Attribute::to_field);
            // Include trailing comma here as we know we have fields
            quote! { #(#entries),*, }
        }
    }

    fn to_init_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(Attribute::to_init);
            // Include trailing semi-colon here as we know we have fields
            quote! { #(#entries);*; }
        }
    }

    fn to_unpack_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().flat_map(Attribute::to_unpack);
            // Include trailing semi-colon here as we know we have fields
            quote! { #(#entries);*; }
        }
    }
}

fn convert_enum_name(name: &str, renames: Option<&Vec<(String, String)>>) -> String {
    if let Some(renames) = renames {
        if let Some(rename_to) = renames
            .iter()
            .find_map(|(from, to)| if from == name { Some(to) } else { None })
        {
            return rename_to.to_string();
        }
    }

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

fn get_attribute_fields(element: &rx::Node) -> Vec<Attribute> {
    element
        .children()
        .filter(|child| child.tag_name().name() == "attribute")
        .flat_map(
            |child| match (child.attribute("name"), child.attribute("type")) {
                (Some(name), Some(type_)) => Some(Attribute {
                    name: name.to_string(),
                    safe_name: convert_field_name(name),
                    type_: Type::from_str(type_),
                    optional: child.attribute("use") == Some("optional"),
                }),
                _ => None,
            },
        )
        .collect::<Vec<_>>()
}

fn create_struct(node: rx::Node, context: &Context) -> anyhow::Result<TokenStream> {
    let type_name = node
        .attribute("name")
        .context("Failed to get name attribute")?;

    if context.skip_types.contains(type_name) {
        return Ok(TokenStream::new());
    }

    let type_name = Type::from_str(
        node.attribute("name")
            .context("Failed to get name attribute")?,
    );

    let type_name_id = type_name.to_type_id();

    let attributes = get_attribute_fields(&node);
    let attribute_fields = attributes.to_fields_stream();
    let attribute_field_names = attributes.to_names_stream();
    let attribute_inits = attributes.to_init_stream();

    let elements = get_elements(&node)?;
    let element_fields = elements.to_fields_stream();
    let element_field_names = elements.to_names_stream();
    let element_mut_inits = elements.to_mut_init_stream();
    let element_inits = elements.to_init_stream();
    let element_unpacks = elements.to_unpack_stream();
    let element_matches = elements.to_matches_stream();
    let empty_element_matches = elements.to_empty_matches_stream();

    Ok(quote! {
        #[derive(Debug)]
        pub struct #type_name_id {
            #attribute_fields
            #element_fields
        }

        impl #type_name_id {
            fn parse(
                reader: &mut Reader<&[u8]>,
                start_tag: BytesStart<'_>,
            ) -> anyhow::Result<Self> {
                tracing::debug!("Parsing {:?}", start_tag.name());
                #attribute_inits
                #element_mut_inits

                loop {
                    match reader.read_event() {
                        Ok(Event::Start(tag)) => match tag.name().as_ref() {
                            #element_matches
                            _ => {}
                        },
                        Ok(Event::Empty(tag)) => match tag.name().as_ref() {
                            #empty_element_matches
                            _ => {}
                        },
                        Ok(Event::End(tag)) => {
                            if tag.name() == start_tag.name() {
                                #element_unpacks
                                return Ok(#type_name_id {
                                    #attribute_field_names
                                    #element_field_names
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }

            fn parse_empty(
                start_tag: BytesStart<'_>,
            ) -> anyhow::Result<Self> {
                tracing::debug!("Parsing {:?}", start_tag.name());
                #attribute_inits
                #element_inits
                #element_unpacks
                Ok(#type_name_id {
                    #attribute_field_names
                    #element_field_names
                })
            }
        }
    })
}

fn create_mixed_content(element: rx::Node) -> anyhow::Result<TokenStream> {
    let type_name = Type::from_str(
        element
            .attribute("name")
            .context("Failed to get name attribute")?,
    );

    let type_name_id = type_name.to_type_id();
    let item_id = id(&format!("{type_name_id}Item"));

    let attributes = get_attribute_fields(&element);

    let mut unexpected_tag = quote! {
        tag_name => {
            return Err(
                anyhow::anyhow!(
                    "unexpected tag {:?} when parsing {}",
                    String::from_utf8_lossy(tag_name),
                    std::any::type_name::<#type_name_id>()
                )
            );
        }
    };

    let mut unexpected_empty_tag = quote! {
        tag_name => {
            return Err(
                anyhow::anyhow!(
                    "unexpected tag {:?} when parsing {}",
                    String::from_utf8_lossy(tag_name),
                    std::any::type_name::<#type_name_id>()
                )
            );
        }
    };

    let mut entries = Vec::new();
    let mut match_entries = Vec::new();
    let mut match_empty_entries = Vec::new();

    for child in element.children() {
        match child.tag_name().name() {
            "sequence" | "choice" => {
                let mut new_enum_entries = Vec::new();
                let mut new_match_entries = Vec::new();
                let mut new_match_empty_entries = Vec::new();

                for grand_child in child.children() {
                    match (grand_child.attribute("name"), grand_child.attribute("type")) {
                        (Some(name), Some(type_)) => {
                            let name_bytes = proc_macro2::Literal::byte_string(name.as_bytes());
                            let name = id(&name.to_upper_camel_case());

                            let type_ = Type::from_str(type_);
                            let type_id = type_.to_type_id();
                            let parse_call = type_.to_parse_call();

                            new_enum_entries.push(quote! {
                                #name(#type_id),
                            });

                            new_match_entries.push(quote! {
                                #name_bytes => {
                                    content.push(#item_id::#name(#parse_call))
                                }
                            });

                            if let Some(parse_empty_call) = type_.to_parse_empty_call() {
                                new_match_empty_entries.push(quote! {
                                    #name_bytes => {
                                        content.push(#item_id::#name(#parse_empty_call))
                                    }
                                });
                            }
                        }
                        _ => {}
                    }
                }

                new_enum_entries.push(quote! { Text(String) });
                entries.append(&mut new_enum_entries);
                match_entries.append(&mut new_match_entries);
                match_empty_entries.append(&mut new_match_empty_entries);
            }
            "group" => {
                if let Some(ref_) = child.attribute("ref") {
                    let type_name = Type::from_str(ref_).to_type_id();
                    entries.push(quote! {
                        #type_name(#type_name),
                        Text(String),
                    });
                    unexpected_tag = quote! {
                        _ => {
                            content.push(#item_id::#type_name(#type_name::parse(reader, tag)?));
                        }
                    };
                    unexpected_empty_tag = quote! {
                        _ => {
                            content.push(#item_id::#type_name(#type_name::parse_empty(tag)?));
                        }
                    };
                }
            }
            _ => {}
        }
    }

    let attribute_fields = attributes.to_fields_stream();
    let attribute_field_names = attributes.to_names_stream();
    let attribute_inits = attributes.to_init_stream();

    if entries.is_empty() {
        Ok(quote! {
            #[derive(Debug)]
            pub struct #type_name_id {
                #attribute_fields
                pub content: String,
            }

            impl #type_name_id {
                fn parse(
                    reader: &mut Reader<&[u8]>,
                    start_tag: BytesStart<'_>,
                ) -> anyhow::Result<Self> {
                    #attribute_inits

                    let mut content = String::new();
                    loop {
                        match reader.read_event() {
                            Ok(Event::Text(text)) => {
                                content = String::from_utf8(text.to_vec()).map_err(|err| anyhow::anyhow!("{:?}", err))?;
                            }
                            Ok(Event::End(tag)) => {
                                if tag.name() == start_tag.name() {
                                    return Ok(#type_name_id {
                                        #attribute_field_names
                                        content
                                    });
                                }
                            }
                            event => return Err(anyhow::anyhow!("unexpected event '{:?}' when parsing '{:?}'", event, start_tag.name())),
                        }
                    }
                }

                #[allow(unused_variables)]
                fn parse_empty(
                    start_tag: BytesStart<'_>,
                ) -> anyhow::Result<Self> {
                    #attribute_inits
                    Ok(#type_name_id {
                        #attribute_field_names
                        content: String::new(),
                    })
                }
            }
        })
    } else {
        Ok(quote! {
            #[derive(Debug)]
            pub struct #type_name_id {
                #attribute_fields
                pub content: Vec<#item_id>,
            }

            #[derive(Debug)]
            pub enum #item_id {
                #(#entries)*
            }

            impl #type_name_id {
                fn parse(
                    reader: &mut Reader<&[u8]>,
                    start_tag: BytesStart<'_>,
                ) -> anyhow::Result<Self> {
                    #attribute_inits
                    let mut content = Vec::new();
                    loop {
                        match reader.read_event() {
                            Ok(Event::Start(tag)) => match tag.name().as_ref() {
                                #(#match_entries)*
                                #unexpected_tag
                            },
                            Ok(Event::Empty(tag)) => match tag.name().as_ref() {
                                #(#match_empty_entries)*
                                #unexpected_empty_tag
                            },
                            Ok(Event::Text(text)) => content.push(#item_id::Text(
                                String::from_utf8(text.to_vec()).map_err(|err| anyhow::anyhow!("{:?}", err))?,
                            )),
                            Ok(Event::End(tag)) => {
                                if tag.name() == start_tag.name() {
                                    return Ok(#type_name_id {
                                        #attribute_field_names
                                        content
                                    });
                                }
                            }
                            event => return Err(anyhow::anyhow!("unexpected event '{:?}' when parsing '{:?}'", event, start_tag.name())),
                        }
                    }
                }

                #[allow(unused_variables)]
                fn parse_empty(
                    start_tag: BytesStart<'_>,
                ) -> anyhow::Result<Self> {
                    #attribute_inits
                    Ok(#type_name_id {
                        #attribute_field_names
                        content: Vec::new(),
                    })
                }
            }
        })
    }
}

fn create_simple_content(element: rx::Node) -> anyhow::Result<TokenStream> {
    let type_name = Type::from_str(
        element
            .attribute("name")
            .context("Unable to get name attribute")?,
    )
    .to_type_id();

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

    let content_type = Type::from_str(content_type);

    let attributes = get_attribute_fields(&extension);
    let attribute_fields = attributes.to_fields_stream();
    let attribute_field_names = attributes.to_names_stream();
    let attribute_inits = attributes.to_init_stream();

    let type_name = type_name.to_token_stream();

    if let Type::String = content_type {
        let type_id = content_type.to_type_id();
        Ok(quote! {
            #[derive(Debug)]
            pub struct #type_name {
                #attribute_fields
                pub content: #type_id,
            }

            impl #type_name {
                fn parse(
                    reader: &mut Reader<&[u8]>,
                    start_tag: BytesStart<'_>,
                ) -> anyhow::Result<Self> {
                    #attribute_inits
                    let mut content = String::new();

                    loop {
                        match reader.read_event() {
                            Ok(Event::Text(text)) => {
                                content = String::from_utf8(text.to_vec()).map_err(|err| anyhow::anyhow!("{:?}", err))?;
                            }
                            Ok(Event::End(tag)) => {
                                if tag.name() == start_tag.name() {
                                    return Ok(#type_name {
                                        #attribute_field_names
                                        content
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                }

                #[allow(unused_variables)]
                fn parse_empty(
                    start_tag: BytesStart<'_>,
                ) -> anyhow::Result<Self> {
                    #attribute_inits
                    Ok(#type_name {
                        #attribute_field_names
                        content: String::new(),
                    })
                }
            }
        })
    } else {
        anyhow::bail!("Unsupported content type for simple content");
    }
}

fn get_elements(element: &rx::Node) -> anyhow::Result<Vec<Element>> {
    let mut elements = Vec::new();

    for child in element.children() {
        match child.tag_name().name() {
            "sequence" => elements.extend(get_elements(&child)?),
            "choice" => elements.extend(get_elements(&child)?),
            "element" => {
                if let Some(name) = child.attribute("name") {
                    let safe_name = convert_field_name(name);
                    // Default to string if no type is present
                    let type_ = Type::from_str(child.attribute("type").unwrap_or("xsd:string"));
                    elements.push(Element {
                        name: name.to_string(),
                        safe_name,
                        type_,
                        wrapper: get_wrapper(&child)?,
                    })
                }
            }
            _ => {}
        }
    }

    Ok(elements)
}

fn create_restriction(
    name: &str,
    node: rx::Node,
    context: &Context,
) -> anyhow::Result<TokenStream> {
    let type_name_id = Type::from_str(name).to_token_stream();

    // TODO: Lift to top configuration or base off xsd info
    if name == "DoxVersionNumber" || name == "DoxCharRange" {
        return Ok(quote! { type #type_name_id = String; });
    }

    let mut entries = Vec::new();
    for child in node.children() {
        if child.tag_name().name() == "enumeration" {
            let entry_name = child.attribute("value").context("Failed to get value")?;

            let renames = context
                .enum_variant_renames
                .iter()
                .find_map(
                    |(target, renames)| {
                        if target == name {
                            Some(renames)
                        } else {
                            None
                        }
                    },
                );

            let safe_entry_name = convert_enum_name(entry_name, renames);
            let entry_name_id = id(&safe_entry_name);
            entries.push(quote! {
                #[strum(serialize = #entry_name)]
                #entry_name_id
            });
        }
    }

    Ok(quote! {
        #[derive(Debug, strum::EnumString, Clone, PartialEq)]
        pub enum #type_name_id {
            #(#entries),*
        }
    })
}

fn handle_complex_type(node: rx::Node, context: &Context) -> anyhow::Result<TokenStream> {
    if is_simple_content(&node) {
        create_simple_content(node)
    } else if is_mixed_content(&node) {
        create_mixed_content(node)
    } else {
        create_struct(node, context)
    }
}

fn handle_simple_type(node: rx::Node, context: &Context) -> anyhow::Result<TokenStream> {
    let mut stream = TokenStream::new();

    let name = node
        .attribute("name")
        .context("Failed to get name for simple type")?;

    for child in node.children() {
        match child.tag_name().name() {
            "restriction" => stream.extend(create_restriction(name, child, context)),
            _ => {}
        }
    }

    Ok(stream)
}

enum Choice {
    Group { type_: String },
    Element { name: String, type_: Option<String> },
}

fn get_choice_entries(node: rx::Node, context: &Context) -> anyhow::Result<Vec<Choice>> {
    let mut choices = Vec::new();

    let mut already_seen = HashSet::new();

    for child in node.children() {
        match child.tag_name().name() {
            "group" => {
                let ref_ = child
                    .attribute("ref")
                    .context("Failed to get ref attribute")?;
                choices.push(Choice::Group {
                    type_: ref_.to_string(),
                })
            }
            "element" => match (child.attribute("name"), child.attribute("type")) {
                (Some(name), Some(type_)) => {
                    let normalised_name = Type::from_str(name).to_type_str();
                    if already_seen.insert(normalised_name) {
                        if context.skip_types.contains(type_) {
                            choices.push(Choice::Element {
                                name: name.to_string(),
                                type_: None,
                            })
                        } else {
                            choices.push(Choice::Element {
                                name: name.to_string(),
                                type_: Some(type_.to_string()),
                            })
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(choices)
}

fn handle_group(element: rx::Node, context: &Context) -> anyhow::Result<TokenStream> {
    let enum_name = element
        .attribute("name")
        .context("Failed to get name attribute in restriction")?;

    let enum_name_id = Type::from_str(enum_name).to_type_id();

    let mut choices = Vec::new();

    for child in element.children() {
        if child.tag_name().name() == "choice" {
            choices.extend(get_choice_entries(child, context)?);
        }
    }

    let enum_entries = choices.iter().map(|choice| match choice {
        Choice::Group { type_ } => {
            let type_id = Type::from_str(type_).to_type_id();
            quote! { #type_id(#type_id) }
        }
        Choice::Element { name, type_ } => {
            let name_id = Type::from_str(name).to_type_id();
            match type_ {
                Some(type_) => {
                    let type_id = Type::from_str(type_).to_type_id();
                    quote! { #name_id(#type_id) }
                }
                None => {
                    quote! { #name_id }
                }
            }
        }
    });

    let direct_matches = choices.iter().flat_map(|choice| match choice {
        Choice::Group { .. } => None,
        Choice::Element { name, type_ } => {
            let enum_entry_name_id = Type::from_str(name).to_type_id();
            let bytes_str = proc_macro2::Literal::byte_string(name.as_bytes());
            match type_ {
                Some(type_) => {
                    let type_parse_call = Type::from_str(type_).to_parse_call();
                    Some(quote! {
                        #bytes_str => {
                            Ok(#enum_name_id::#enum_entry_name_id(#type_parse_call))
                        }
                    })
                }
                None => Some(quote! {
                    #bytes_str => {
                        Ok(#enum_name_id::#enum_entry_name_id)
                    }
                }),
            }
        }
    });

    let direct_empty_matches = choices.iter().flat_map(|choice| match choice {
        Choice::Group { .. } => None,
        Choice::Element { name, type_ } => {
            let enum_entry_name_id = Type::from_str(name).to_type_id();
            let bytes_str = proc_macro2::Literal::byte_string(name.as_bytes());
            match type_ {
                Some(type_) => {
                    let type_parse_call = Type::from_str(type_).to_parse_empty_call();
                    Some(quote! {
                        #bytes_str => {
                            Ok(#enum_name_id::#enum_entry_name_id(#type_parse_call))
                        }
                    })
                }
                None => Some(quote! {
                    #bytes_str => {
                        Ok(#enum_name_id::#enum_entry_name_id)
                    }
                }),
            }
        }
    });

    let group_matches = choices.iter().flat_map(|choice| match choice {
        Choice::Group { type_ } => {
            let type_id = Type::from_str(&type_).to_type_id();
            Some(quote! {
                _ => {
                    Ok(#enum_name_id::#type_id(#type_id::parse(reader, tag)?))
                }
            })
        }
        Choice::Element { .. } => None,
    });

    let group_empty_matches = choices.iter().flat_map(|choice| match choice {
        Choice::Group { type_ } => {
            let type_id = Type::from_str(&type_).to_type_id();
            Some(quote! {
                _ => {
                    Ok(#enum_name_id::#type_id(#type_id::parse_empty(tag)?))
                }
            })
        }
        Choice::Element { .. } => None,
    });

    let mut match_unexpected = quote! {
        _ => anyhow::bail!("Unexpected tag")
    };

    let mut match_unexpected_empty = quote! {
        _ => anyhow::bail!("Unexpected tag")
    };

    if choices
        .iter()
        .any(|choice| matches!(choice, Choice::Group { .. }))
    {
        match_unexpected = quote! {};
        match_unexpected_empty = quote! {};
    }

    Ok(quote! {
        #[derive(Debug)]
        pub enum #enum_name_id {
            #(#enum_entries),*
        }

        impl #enum_name_id {
            #[allow(unused_variables)]
            fn parse(
                reader: &mut Reader<&[u8]>,
                tag: BytesStart<'_>,
            ) -> anyhow::Result<Self> {
                match tag.name().as_ref() {
                    #(#direct_matches)*
                    #(#group_matches)*
                    #match_unexpected
                }
            }

            #[allow(unused_variables)]
            fn parse_empty(
                tag: BytesStart<'_>,
            ) -> anyhow::Result<Self> {
                match tag.name().as_ref() {
                    #(#direct_empty_matches)*
                    #(#group_empty_matches)*
                    #match_unexpected_empty
                }
            }
        }
    })
}

struct Context {
    skip_types: HashSet<String>,
    enum_variant_renames: EnumVariantRenames,
}

pub struct Root {
    pub tag: String,
    pub type_: String,
}

fn generate(context: Context, root: Root, xsd_str: &str) -> anyhow::Result<String> {
    let doc = rx::Document::parse(&xsd_str)?;

    let schema = doc
        .root()
        .first_element_child()
        .context("Failed to get first element")?;

    let mut nodes = TokenStream::new();

    for child in schema.children() {
        match child.tag_name().name() {
            "complexType" => nodes.extend(handle_complex_type(child, &context)?),
            "simpleType" => nodes.extend(handle_simple_type(child, &context)?),
            "group" => nodes.extend(handle_group(child, &context)?),
            _ => {}
        }
    }

    let root_tag_literal = proc_macro2::Literal::byte_string(root.tag.as_bytes());
    let root_type = id(&root.type_);

    let file_ast = quote! {
        use anyhow::Context;
        use quick_xml::events::{BytesStart, Event};
        use quick_xml::reader::Reader;

        use crate::xml;

        pub fn parse(xml: &str) -> anyhow::Result<#root_type> {
            let mut reader = Reader::from_str(xml);

            loop {
                match reader.read_event() {
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),

                    Ok(Event::Eof) => {}

                    Ok(Event::Start(tag)) => {
                        if let #root_tag_literal = tag.name().as_ref() {
                            return #root_type::parse(&mut reader, tag)
                        }
                    }

                    _ => (),
                }
            }
        }

        #nodes
    };

    Ok(file_ast.to_token_stream().to_string())
}

type EnumVariantRenames = Vec<(String, Vec<(String, String)>)>;

pub struct Builder {
    root: Root,
    path: PathBuf,
    module: Option<String>,
    enum_variant_renames: EnumVariantRenames,
    skip_types: HashSet<String>,
}

impl Builder {
    pub fn new(path: PathBuf, root: Root) -> Self {
        Self {
            path,
            root,
            module: None,
            enum_variant_renames: Vec::new(),
            skip_types: HashSet::new(),
        }
    }

    pub fn module(mut self, name: &str) -> Self {
        self.module = Some(name.to_string());
        self
    }

    pub fn rename_enum_variants(mut self, enum_variant_renames: EnumVariantRenames) -> Self {
        self.enum_variant_renames = enum_variant_renames;
        self
    }

    pub fn skip_types(mut self, skip_types: HashSet<String>) -> Self {
        self.skip_types = skip_types;
        self
    }

    pub fn generate(self) -> anyhow::Result<PathBuf> {
        let module = match self.module {
            Some(name) => name,
            None => self
                .path
                .file_stem()
                .context("Failed to extract module name from xsd path")?
                .to_string_lossy()
                .to_owned()
                .to_string(),
        };

        let context = Context {
            enum_variant_renames: self.enum_variant_renames,
            skip_types: self.skip_types,
        };

        let xsd_str = std::fs::read_to_string(&self.path)?;
        let code_string = generate(context, self.root, &xsd_str)?;

        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
        let dir = out_dir.join("xsds");

        std::fs::create_dir_all(&dir)?;

        let path = dir.join(format!("{module}.rs"));

        std::fs::write(&path, code_string)?;

        Ok(path)
    }
}
