//! Helper functions designed for rendering doxygen elements to text specifically with an eye to
//! the inputs required for Sphinx domain directives. If we need other text based targets then we might
//! want to introduce changes to how the rendering is done in places

use crate::doxygen::compound::generated as e;
use crate::Domain;

pub fn render_compound_def(domain: &Domain, compound_def: &e::CompounddefType) -> String {
    match domain {
        Domain::CPlusPlus => compound_def.compoundname.to_string(),
        // For C, for Sphinx, we want to express it with a '.' instead of '::'
        Domain::C => compound_def.compoundname.replace("::", "."),
    }
}

pub fn render_compound_kind(kind: &e::DoxCompoundKind) -> &'static str {
    match kind {
        e::DoxCompoundKind::Class => "class",
        e::DoxCompoundKind::Struct => "struct",
        e::DoxCompoundKind::Union => "union",
        e::DoxCompoundKind::Interface => "interface",
        e::DoxCompoundKind::Protocol => "protocol",
        e::DoxCompoundKind::Category => "category",
        e::DoxCompoundKind::Exception => "exception",
        e::DoxCompoundKind::Service => "service",
        e::DoxCompoundKind::Singleton => "singleton",
        e::DoxCompoundKind::Module => "module",
        e::DoxCompoundKind::Type => "type",
        e::DoxCompoundKind::File => "file",
        e::DoxCompoundKind::Namespace => "namespace",
        e::DoxCompoundKind::Group => "group",
        e::DoxCompoundKind::Page => "page",
        e::DoxCompoundKind::Example => "example",
        e::DoxCompoundKind::Dir => "dir",
        e::DoxCompoundKind::Concept => "concept",
    }
}

fn with_trailing(mut string: String, suffix: &str) -> String {
    string.push_str(suffix);
    string
}

fn with_leading(mut string: String, char: char) -> String {
    string.insert(0, char);
    string
}

fn if_yes(dox_bool: &e::DoxBool, str: &str) -> Option<String> {
    if dox_bool == &e::DoxBool::Yes {
        Some(str.to_string())
    } else {
        None
    }
}

pub fn render_member_def(domain: &Domain, member_def: &e::MemberdefType) -> String {
    match member_def.kind {
        e::DoxMemberKind::Function => [
            if_yes(&member_def.static_, "static "),
            if_yes(
                member_def.inline.as_ref().unwrap_or(&e::DoxBool::No),
                "inline ",
            ),
            if_yes(
                member_def.const_.as_ref().unwrap_or(&e::DoxBool::No),
                "const ",
            ),
            member_def
                .type_
                .as_ref()
                .map(render_linked_text_type)
                .map(|str| with_trailing(str, " ")),
            match domain {
                // If we're in the C++ domain then try to use the qualified name if possible so that it registers
                // as a class member when it is a class member
                Domain::CPlusPlus => Some(
                    member_def
                        .qualifiedname
                        .as_ref()
                        .unwrap_or(&member_def.name)
                        .clone(),
                ),
                Domain::C => Some(member_def.name.clone()),
            },
            member_def.argsstring.clone(),
        ]
        .into_iter()
        .flatten()
        .collect(),
        e::DoxMemberKind::Define => {
            if member_def.param.is_empty() {
                member_def.name.clone()
            } else {
                let params: Vec<String> = member_def
                    .param
                    .iter()
                    .flat_map(|param| param.defname.clone())
                    .collect();
                let params = params.join(", ");
                format!("{}({})", member_def.name, params)
            }
        }
        e::DoxMemberKind::Enum => {
            // Some enums are anonymous so we use the name if it isn't empty but otherwise we use the auto-generated
            // id from doxygen with an '@' at the start as '@' is how Sphinx distinguishes anonymous entries
            if !member_def.name.is_empty() {
                member_def.name.clone()
            } else {
                format!("@{}", member_def.id)
            }
        }
        e::DoxMemberKind::Typedef => [
            member_def.type_.as_ref().map(render_linked_text_type),
            Some(member_def.name.clone()),
            member_def.argsstring.clone(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" "),
        e::DoxMemberKind::Variable => {
            let name = member_def
                .qualifiedname
                .as_ref()
                .map(|name| match domain {
                    // Sphinx expects C++ entries to be 'type Struct::member'
                    Domain::CPlusPlus => name.clone(),
                    // Sphinx expects C entries to be 'type Struct.member'
                    Domain::C => name.replace("::", "."),
                })
                .unwrap_or_else(|| member_def.name.clone());

            [
                member_def
                    .type_
                    .as_ref()
                    .map(render_linked_text_type)
                    .map(|type_| match domain {
                        Domain::CPlusPlus => type_,
                        // For structs nested in other structs the type can render to something with a '::' separator
                        // in it and like with other situations we want '.' separators for C code.
                        Domain::C => type_.replace("::", "."),
                    }),
                Some(name),
                member_def.argsstring.as_ref().and_then(|str| {
                    // If the argsstring starts with a ')' then it is likely the args for a function pointer and so
                    // we should include them in the output, otherwise we skip them as they might include complex
                    // expressions which cause issues
                    if str.starts_with(')') {
                        Some(html_escape::decode_html_entities(str).to_string())
                    } else {
                        None
                    }
                }),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" ")
        }
        _ => todo!(
            "text::render_member_def not implemented for {:?}",
            member_def.kind
        ),
    }
}

pub fn render_enum_value(
    domain: &Domain,
    enum_name_or_anon_id: &str,
    enum_value: &e::EnumvalueType,
) -> String {
    let separator = match domain {
        // Use the enum name as a qualifier with :: if we're in the C++ domain
        Domain::CPlusPlus => "::",
        // Use the enum name as a qualifier with . if we're in the C domain
        Domain::C => ".",
    };

    [
        option_from_str(enum_name_or_anon_id).map(|str| with_trailing(str.to_string(), separator)),
        Some(enum_value.name.to_string()),
        enum_value
            .initializer
            .as_ref()
            .map(render_linked_text_type)
            .map(|str| with_leading(str, ' '))
            // Some initializers have new lines which make it into the XML but confuse Sphinx
            .map(|str| collapse_lines(&str)),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn collapse_lines(str: &str) -> String {
    str.replace('\n', " ")
}

fn option_from_str(str: &str) -> Option<&str> {
    if str.is_empty() {
        None
    } else {
        Some(str)
    }
}

pub fn render_linked_text_type(linked_text_type: &e::LinkedTextType) -> String {
    let mut nodes = Vec::new();

    for entry in linked_text_type.content.iter() {
        match entry {
            e::LinkedTextTypeItem::Ref(ref content) => nodes.push(render_ref_text_type(content)),
            e::LinkedTextTypeItem::Text(text) => {
                // HTML escaping is primarily done for enumerator initializers which seem to have html entities in
                // them quite often. It is likely that it is also useful for other blocks of text but we generally
                // don't render other blocks of of text with this set of 'text' functions as they are mostly focused
                // on prepping text for signatures for Sphinx domain arguments
                nodes.push(html_escape::decode_html_entities(text).to_string())
            }
        }
    }

    nodes.join(" ")
}

fn render_ref_text_type(ref_text_type: &e::RefTextType) -> String {
    // HTML escaping is primarily done for enumerator initializers which seem to have html entities in
    // them quite often. It is likely that it is also useful for other blocks of text but we generally
    // don't render other blocks of of text with this set of 'text' functions as they are mostly focused
    // on prepping text for signatures for Sphinx domain arguments
    html_escape::decode_html_entities(&ref_text_type.content).to_string()
}
