//! Helper functions designed for rendering doxygen elements to text specifically with an eye to
//! the inputs required for Sphinx domain directives. If we need other text based targets then we might
//! want to introduce changes to how the rendering is done in places

use crate::doxygen::compound::generated as e;
use crate::Domain;

pub fn render_compound_def(compound_def: &e::CompounddefType) -> String {
    // format!("{} {}", render_compound_kind(&compound_def.kind), compound_def.compoundname)
    compound_def.compoundname.to_string()
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

fn with_trailing_space(mut string: String) -> String {
    string.push(' ');
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
                &member_def.inline.as_ref().unwrap_or(&e::DoxBool::No),
                "inline ",
            ),
            if_yes(
                &member_def.const_.as_ref().unwrap_or(&e::DoxBool::No),
                "const ",
            ),
            member_def
                .type_
                .as_ref()
                .map(render_linked_text_type)
                .map(with_trailing_space),
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
        e::DoxMemberKind::Typedef => member_def
            .definition
            .as_ref()
            .map(|definition| {
                definition
                    // Remove the 'typedef ' prefix as the Sphinx C-Domain doesn't expect to see it
                    .strip_prefix("typedef ")
                    .unwrap_or(definition)
                    .to_string()
            })
            // Note: This could be improved but we expect there to always be a definition
            // in this case
            .unwrap_or_else(|| member_def.name.clone()),
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
                member_def.type_.as_ref().map(render_linked_text_type),
                Some(name),
                member_def.argsstring.clone(),
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
    enum_name: &str,
    enum_value: &e::EnumvalueType,
) -> String {
    match domain {
        // Use the enum name as a qualifier if we're in the C++ domain
        Domain::CPlusPlus => {
            if enum_name.is_empty() {
                enum_value.name.to_string()
            } else {
                // Use the enum name as a qualifier if we're in the C++ domain
                format!("{enum_name}::{}", enum_value.name)
            }
        }
        // Otherwise all we want is the enumerator name
        Domain::C => {
            if enum_name.is_empty() {
                enum_value.name.to_string()
            } else {
                format!("{enum_name}.{}", enum_value.name)
            }
        }
    }
}

pub fn render_linked_text_type(linked_text_type: &e::LinkedTextType) -> String {
    let mut nodes = Vec::new();

    for entry in linked_text_type.content.iter() {
        match entry {
            e::LinkedTextTypeItem::Ref(ref content) => nodes.push(render_ref_text_type(content)),
            e::LinkedTextTypeItem::Text(text) => nodes.push(text.clone()),
        }
    }

    nodes.join(" ")
}

fn render_ref_text_type(ref_text_type: &e::RefTextType) -> String {
    ref_text_type.content.clone()
}
