use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::Context as AnyhowContext;
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use roxmltree as rx;

trait ToTokenStream {
    fn to_names_stream(&self) -> TokenStream;
    fn to_fields_stream(&self) -> TokenStream;
    fn to_init_stream(&self) -> TokenStream;
    fn to_unpack_stream(&self) -> TokenStream;
}

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
        _ => String::from(str.to_upper_camel_case()),
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

fn create_struct(node: rx::Node) -> anyhow::Result<TokenStream> {
    let type_name = id(&convert_type(
        node.attribute("name")
            .context("Failed to get name attribute")?,
    ));

    // Skip DocEmptyType as it represents nothing
    if type_name == "DocEmptyType" {
        return Ok(TokenStream::new());
    }

    let attributes = get_attribute_fields(&node);
    let attribute_fields = attributes.to_fields_stream();
    let attribute_field_names = attributes.to_names_stream();
    let attribute_inits = attributes.to_init_stream();
    // let attribute_unpacks = attributes.to_unpack_stream();

    let elements = get_elements(&node)?;
    let element_fields = elements.to_fields_stream();
    let element_field_names = elements.to_names_stream();
    let element_inits = elements.to_init_stream();
    let element_unpacks = elements.to_unpack_stream();

    Ok(quote! {
        struct #type_name {
            #attribute_fields
            #element_fields
        }

        impl #type_name {
            fn parse(
                reader: &mut Reader<&[u8]>,
                start_tag: BytesStart<'_>,
            ) -> anyhow::Result<Self> {
                #attribute_inits
                #element_inits

                loop {
                    match reader.read_event() {
                        Ok(Event::Start(tag)) => match tag.name().as_ref() {
                            /*
                            b"compoundname" => {
                                compound_name = xml::parse_text(reader)?;
                            }
                            b"briefdescription" => {
                                brief_description = Some(parse_description(reader, tag)?);
                            }
                            b"detaileddescription" => {
                                detailed_description = Some(parse_description(reader, tag)?);
                            }
                            b"sectiondef" => {
                                section_defs.push(parse_section_def(reader, tag)?);
                            }
                            _ => {}
                            */
                        },
                        Ok(Event::End(tag)) => {
                            if tag.name() == start_tag.name() {
                                #element_unpacks
                                return Ok(#type_name {
                                    #attribute_field_names
                                    #element_field_names
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
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

    let attributes_token_stream = attributes.to_fields_stream();

    if entries.is_empty() {
        Ok(quote! {
            struct #name_id {
                #attributes_token_stream
                pub content: String,
            }
        })
    } else {
        let item_id = id(&format!("{name}Item"));
        Ok(quote! {
            struct #name_id {
                #attributes_token_stream
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
    let attributes_token_stream = attributes.to_fields_stream();

    Ok(quote! {
        struct #name {
            #attributes_token_stream
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

struct Element {
    name: Ident,
    wrapper: Option<Wrapper>,
    type_: TokenStream,
}

impl Element {
    fn to_field(&self) -> TokenStream {
        let name = self.name.clone();
        let type_ = self.type_.clone();

        match self.wrapper {
            Some(Wrapper::Vec) => {
                quote! { #name: Vec<#type_> }
            }
            Some(Wrapper::Vec1) => {
                quote! { #name: vec1::Vec1<#type_> }
            }
            Some(Wrapper::Option) => {
                quote! { #name: Option<#type_> }
            }
            None => {
                quote! { #name: #type_ }
            }
        }
    }

    fn to_init(&self) -> TokenStream {
        let name = self.name.clone();
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

    fn to_unpack(&self) -> Option<TokenStream> {
        let name = self.name.clone();

        match self.wrapper {
            Some(Wrapper::Vec) => None,
            Some(Wrapper::Vec1) => Some(
                quote! { let #name = vec1::Vec1::try_from_vec(#name).context("Vec was empty")?; },
            ),
            Some(Wrapper::Option) => None,
            None => Some(quote! { let #name = #name.context("Failed to find value")?; }),
        }
    }
}

impl ToTokenStream for Vec<Element> {
    fn to_names_stream(&self) -> TokenStream {
        if self.is_empty() {
            TokenStream::new()
        } else {
            let entries = self.iter().map(|element| element.name.clone());
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
}

#[derive(Debug, Clone)]
enum Type {
    Bool,
    Integer,
    String,
    Enum(String),
}

impl Type {
    fn from_str(str: &str) -> Self {
        match str {
            "DoxBool" => Self::Bool,
            "xsd:integer" => Self::Integer,
            "xsd:string" => Self::String,
            _ => Self::Enum(str.to_string()),
        }
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            Self::Bool => quote! { bool },
            Self::Integer => quote! { i32 },
            Self::String => quote! { String },
            Self::Enum(name) => {
                let name = id(&name);
                quote! { #name }
            }
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
            quote! { #name: Option<#type_> }
        } else {
            quote! { #name: #type_ }
        }
    }

    fn to_init(&self) -> TokenStream {
        let field_name = id(&self.safe_name);
        let attr_name = proc_macro2::Literal::byte_string(self.name.as_bytes());
        // TODO: Move these extra parse code into xml module helpers rather than having it inline here
        match self.type_ {
            Type::Bool => {
                if self.optional {
                    quote! {
                        let #field_name = xml::get_optional_attribute_string(#attr_name, &start_tag)?
                                                .map(|str| str.parse::<bool>()).transpose()?
                    }
                } else {
                    quote! {
                        let #field_name = xml::get_attribute_string(#attr_name, &start_tag)?.parse::<bool>()?
                    }
                }
            }
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

impl ToTokenStream for Vec<Attribute> {
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

fn get_elements(element: &rx::Node) -> anyhow::Result<Vec<Element>> {
    let mut elements = Vec::new();

    for child in element.children() {
        match child.tag_name().name() {
            "sequence" => elements.extend(get_elements(&child)?),
            "choice" => elements.extend(get_elements(&child)?),
            "element" => {
                if let Some(name) = child.attribute("name") {
                    let name = id(&convert_field_name(name));
                    let type_ = id(&convert_type(child.attribute("type").unwrap_or("String")));
                    elements.push(Element {
                        name,
                        type_: quote! { #type_ },
                        wrapper: get_wrapper(&child)?,
                    })

                    /*
                    match get_wrapper(&child)? {
                        Some(Wrapper::Vec) => elements.push(Element {
                            name,
                            type_: quote! { Vec<#type_> },
                        }),
                        Some(Wrapper::Vec1) => elements.push(Element {
                            name,
                            type_: quote! { vec1::Vec1<#type_>, },
                        }),
                        Some(Wrapper::Option) => elements.push(Element {
                            name,
                            type_: quote! { Option<#type_>, },
                        }),
                        None => elements.push(Element {
                            name,
                            type_: quote! { #type_ },
                        }),
                    }
                    */
                }
            }
            _ => {}
        }
    }

    Ok(elements)
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

fn create_restriction(
    name: &str,
    node: rx::Node,
    context: &Context,
) -> anyhow::Result<TokenStream> {
    let name_id = id(&convert_type(name));

    if name == "DoxVersionNumber" || name == "DoxCharRange" {
        return Ok(quote! { type #name_id = String; });
    }

    if name == "DoxBool" {
        return Ok(TokenStream::new());
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

            let entry_name = convert_enum_name(entry_name, renames);

            entries.push(id(&entry_name));
        }
    }

    Ok(quote! {
        #[derive(Debug, strum::EnumString, Clone)]
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

struct Context {
    enum_variant_renames: EnumVariantRenames,
}

fn generate_mod(
    context: Context,
    root_tag: &str,
    root_type: &str,
    mod_name: &str,
    xsd_path: &Path,
) -> anyhow::Result<()> {
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
            "simpleType" => nodes.extend(handle_simple_type(child, &context)?),
            "group" => nodes.extend(handle_group(child)?),
            _ => {}
        }
    }

    let root_tag_literal = proc_macro2::Literal::byte_string(root_tag.as_bytes());
    let root_type = id(root_type);

    let file_ast = quote! {
        #[allow(dead_code)]
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

type EnumVariantRenames = Vec<(String, Vec<(String, String)>)>;

pub struct Builder {
    root_tag: String,
    root_type: String,
    path: PathBuf,
    module: Option<String>,
    enum_variant_renames: Option<EnumVariantRenames>,
}

impl Builder {
    pub fn new(path: PathBuf, root_tag: String, root_type: String) -> Self {
        Self {
            path,
            root_tag,
            root_type,
            module: None,
            enum_variant_renames: None,
        }
    }

    pub fn module(mut self, name: &str) -> Self {
        self.module = Some(name.to_string());
        self
    }

    pub fn rename_enum_variants(mut self, enum_variant_renames: EnumVariantRenames) -> Self {
        self.enum_variant_renames = Some(enum_variant_renames);
        self
    }

    pub fn generate(self) -> anyhow::Result<()> {
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
            enum_variant_renames: self.enum_variant_renames.unwrap_or_default(),
        };

        generate_mod(
            context,
            &self.root_tag,
            &self.root_type,
            &module,
            &self.path,
        )
    }
}
